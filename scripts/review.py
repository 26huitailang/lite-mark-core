#!/usr/bin/env python3
"""
Kimi Code Review - 本地/CI 通用代码审查脚本（LiteMark 定制版）

用法:
  python scripts/review.py local              # 审查当前未提交的变更
  python scripts/review.py staged             # 审查已暂存的变更
  python scripts/review.py pr <base_branch>   # 审查当前分支与 base 的差异
  python scripts/review.py file <path>        # 审查单个文件
  python scripts/review.py diff <diff_file>   # 审查已有的 diff 文件

环境变量:
  KIMI_AGENT_FILE    自定义 Agent 文件路径（默认: .kimi/agents/reviewer.yaml）
  KIMI_MODEL         指定模型（默认使用配置）
  REVIEW_MAX_DIFF    diff 最大行数，超长的会截断（默认: 800）
  REVIEW_OUTPUT      输出文件路径（默认: stdout）
  REVIEW_JSON_OUTPUT 结构化 JSON 输出路径（默认: review_result.json）
"""

import json
import os
import re
import subprocess
import sys
from pathlib import Path
from typing import Optional


DEFAULT_AGENT = ".kimi/agents/reviewer.yaml"
DEFAULT_MAX_DIFF = 800


def run_git(args: list[str]) -> str:
    result = subprocess.run(
        ["git"] + args, capture_output=True, text=True, check=True
    )
    return result.stdout


def get_diff_local(mode: str, target: Optional[str] = None) -> tuple[str, dict]:
    """获取本地 diff，同时返回统计信息"""
    if mode == "local":
        diff = run_git(["diff"])
        stat = run_git(["diff", "--stat"])
    elif mode == "staged":
        diff = run_git(["diff", "--staged"])
        stat = run_git(["diff", "--staged", "--stat"])
    elif mode == "pr":
        base = target or "main"
        # 确保 base 分支存在（可能是 origin/main）
        try:
            run_git(["rev-parse", "--verify", base])
        except subprocess.CalledProcessError:
            base = f"origin/{base}"
        diff = run_git(["diff", f"{base}...HEAD"])
        stat = run_git(["diff", f"{base}...HEAD", "--stat"])
    elif mode == "file":
        if not target:
            raise ValueError("file 模式需要提供文件路径")
        diff = run_git(["diff", "HEAD", "--", target])
        if not diff:
            # 可能是新增文件
            diff = run_git(["show", f"HEAD:{target}"]) if run_git(["ls-files", target]) else ""
        stat = f"{target} | 1 +"
    elif mode == "diff":
        if not target:
            raise ValueError("diff 模式需要提供 diff 文件路径")
        with open(target, "r") as f:
            diff = f.read()
        stat = ""
    else:
        raise ValueError(f"未知模式: {mode}")

    stats = parse_stat(stat)
    return diff, stats


def parse_stat(stat_text: str) -> dict:
    """解析 git diff --stat 输出"""
    files = 0
    insertions = 0
    deletions = 0
    for line in stat_text.strip().splitlines():
        # 匹配:  src/foo.py | 10 ++++++----
        m = re.search(r"\|\s+(\d+)\s+([\+\-]*)", line)
        if m:
            files += 1
            changes = m.group(2)
            insertions += changes.count("+")
            deletions += changes.count("-")
        # 匹配最后一行:  3 files changed, 12 insertions(+), 5 deletions(-)
        m2 = re.search(r"(\d+) files? changed.*?(\d+) insertions?.*?(\d+) deletions?", line)
        if m2:
            files = int(m2.group(1))
            insertions = int(m2.group(2))
            deletions = int(m2.group(3))
    return {"files_changed": files, "lines_added": insertions, "lines_removed": deletions}


def truncate_diff(diff: str, max_lines: int) -> str:
    """截断过长的 diff，保留文件头部和变更概览"""
    lines = diff.splitlines()
    if len(lines) <= max_lines:
        return diff

    # 保留前 max_lines 行，并在末尾添加截断说明
    kept = lines[:max_lines]
    # 尝试保留完整的文件块
    # 从截断处往前找，找到最后一个 diff --git 或 @@ 开头，截到那里
    last_good = max_lines
    for i in range(max_lines - 1, max_lines - 100, -1):
        if i < 0:
            break
        if lines[i].startswith("diff --git"):
            last_good = i
            break

    result_lines = lines[:last_good]
    skipped = len(lines) - len(result_lines)
    result_lines.append("")
    result_lines.append(f"# ... diff 过长，已截断，跳过了 {skipped} 行 ...")
    result_lines.append(f"# 原始 diff 共 {len(lines)} 行，建议分文件审查或缩小变更范围")
    return "\n".join(result_lines)


def build_prompt(diff: str, stats: dict) -> str:
    """构建给 Kimi 的审查 prompt"""
    return f"""请对以下代码变更进行审查。

## 变更统计
- 文件数: {stats['files_changed']}
- 新增行: {stats['lines_added']}
- 删除行: {stats['lines_removed']}

## Diff 内容
```diff
{diff}
```

请严格按照系统提示词要求的 JSON 格式输出审查结果。"""


def run_kimi_review(prompt: str, agent_file: str, model: Optional[str] = None) -> dict:
    """调用 kimi cli 进行审查"""
    cmd = [
        "kimi",
        "--agent-file", agent_file,
        "-p", prompt,
        "--print", "--final-message-only", "--yolo",
    ]
    if model:
        cmd.extend(["-m", model])

    print(f"[review] Running: {' '.join(cmd[:4])} ...", file=sys.stderr)
    try:
        result = subprocess.run(
            cmd, capture_output=True, text=True, stdin=subprocess.DEVNULL, timeout=300
        )
    except subprocess.TimeoutExpired as e:
        print(
            f"[review] kimi 执行超时（300s），diff 可能过大。"
            f"建议：减小变更范围、降低 REVIEW_MAX_DIFF，或分文件审查。",
            file=sys.stderr,
        )
        # 尝试用已捕获的输出兜底
        raw = (e.stdout or "").decode("utf-8", errors="ignore") if isinstance(e.stdout, bytes) else (e.stdout or "")
        return extract_json(raw)

    if result.returncode != 0:
        print(f"[review] kimi 执行失败 (exit={result.returncode}):", file=sys.stderr)
        if result.stdout:
            print(f"[review] stdout:\n{result.stdout[:2000]}", file=sys.stderr)
        if result.stderr:
            print(f"[review] stderr:\n{result.stderr[:2000]}", file=sys.stderr)
        sys.exit(1)

    raw = result.stdout.strip()
    # 过滤掉 resume hint 行
    lines = raw.splitlines()
    filtered_lines = [l for l in lines if not l.startswith("To resume this session:")]
    raw = "\n".join(filtered_lines).strip()
    # 尝试从输出中提取 JSON（可能夹杂其他文本）
    return extract_json(raw)


def extract_json(text: str) -> dict:
    """从文本中提取 JSON 对象"""
    # 先尝试直接解析
    try:
        return json.loads(text)
    except json.JSONDecodeError:
        pass

    # 尝试提取 ```json ... ``` 或 ``` ... ``` 中的内容
    code_block = re.search(r"```(?:json)?\s*(\{.*?)```", text, re.DOTALL)
    if code_block:
        try:
            return json.loads(code_block.group(1))
        except json.JSONDecodeError:
            pass

    # 尝试找到第一个 { 和最后一个 }
    start = text.find("{")
    end = text.rfind("}")
    if start != -1 and end != -1 and end > start:
        try:
            return json.loads(text[start : end + 1])
        except json.JSONDecodeError:
            pass

    # 兜底：返回一个结构化的错误对象
    return {
        "summary": "解析审查结果失败",
        "stats": {"files_changed": 0, "lines_added": 0, "lines_removed": 0},
        "issues": [
            {
                "severity": "medium",
                "category": "maintainability",
                "file": "unknown",
                "line": "unknown",
                "description": "Kimi 输出无法解析为 JSON",
                "suggestion": "请检查 diff 大小或手动运行审查",
            }
        ],
        "approve": False,
        "comment": f"原始输出前 500 字符:\n{text[:500]}",
    }


def generate_markdown_report(result: dict) -> str:
    """生成 Markdown 格式的审查报告"""
    md = []
    md.append("# 🤖 Kimi Code Review Report")
    md.append("")
    md.append(f"**Summary**: {result['summary']}")
    md.append("")

    stats = result.get("stats", {})
    md.append(f"📊 **Stats**: {stats.get('files_changed', 0)} files changed, "
              f"+{stats.get('lines_added', 0)}/-{stats.get('lines_removed', 0)} lines")
    md.append("")

    issues = result.get("issues", [])
    if issues:
        # 按严重程度分组
        severity_order = {"high": 0, "medium": 1, "low": 2}
        issues.sort(key=lambda x: severity_order.get(x.get("severity", "low"), 3))

        md.append(f"## ⚠️ Found {len(issues)} issue(s)")
        md.append("")

        for issue in issues:
            severity = issue.get("severity", "low").upper()
            emoji = {"HIGH": "🔴", "MEDIUM": "🟡", "LOW": "🟢"}.get(severity, "⚪")
            md.append(f"### {emoji} [{severity}] `{issue.get('file', 'unknown')}:{issue.get('line', 'unknown')}`")
            md.append(f"- **Category**: {issue.get('category', 'unknown')}")
            md.append(f"- **Description**: {issue.get('description', '')}")
            md.append(f"- **Suggestion**: {issue.get('suggestion', '')}")
            md.append("")
    else:
        md.append("## ✅ No issues found")
        md.append("")

    status = "✅ Approve" if result.get("approve") else "❌ Request Changes"
    md.append(f"## Verdict: {status}")
    md.append("")
    md.append(f"💬 **Comment**: {result.get('comment', '')}")
    md.append("")

    return "\n".join(md)


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    mode = sys.argv[1]
    target = sys.argv[2] if len(sys.argv) > 2 else None

    agent_file = os.environ.get("KIMI_AGENT_FILE", DEFAULT_AGENT)
    model = os.environ.get("KIMI_MODEL")
    max_diff = int(os.environ.get("REVIEW_MAX_DIFF", DEFAULT_MAX_DIFF))
    output_path = os.environ.get("REVIEW_OUTPUT")
    json_output = os.environ.get("REVIEW_JSON_OUTPUT", "review_result.json")

    # 如果 agent 文件是相对路径，基于项目根目录解析
    if not Path(agent_file).is_absolute():
        # 脚本在 scripts/ 下，项目根目录是上级
        root = Path(__file__).parent.parent
        agent_file = str(root / agent_file)

    print(f"[review] Mode: {mode}, Target: {target}", file=sys.stderr)

    # 1. 获取 diff
    diff, stats = get_diff_local(mode, target)
    if not diff.strip():
        print("[review] No diff found. Nothing to review.", file=sys.stderr)
        sys.exit(0)

    print(f"[review] Diff lines: {len(diff.splitlines())}, truncating to {max_diff}", file=sys.stderr)
    diff = truncate_diff(diff, max_diff)

    # 2. 构建 prompt
    prompt = build_prompt(diff, stats)

    # 3. 运行审查
    result = run_kimi_review(prompt, agent_file, model)

    # 4. 保存 JSON
    with open(json_output, "w", encoding="utf-8") as f:
        json.dump(result, f, ensure_ascii=False, indent=2)
    print(f"[review] JSON saved to: {json_output}", file=sys.stderr)

    # 5. 生成并输出报告
    report = generate_markdown_report(result)

    if output_path:
        with open(output_path, "w", encoding="utf-8") as f:
            f.write(report)
        print(f"[review] Report saved to: {output_path}", file=sys.stderr)
    else:
        print(report)

    # 6. 如果有 high severity 问题，返回非 0 退出码（可用于 CI 阻断）
    has_high = any(i.get("severity") == "high" for i in result.get("issues", []))
    if has_high:
        print("[review] Found HIGH severity issues. Failing.", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
