#!/usr/bin/env bash
# Kimi Code Review - 本地快捷入口（LiteMark 定制版）
# 用法: ./scripts/review.sh [local|staged|pr|file <path>]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$ROOT_DIR"

MODE="${1:-local}"
TARGET="${2:-}"

# 颜色
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

case "$MODE" in
  local)
    echo -e "${YELLOW}🔍 审查当前未提交的变更...${NC}"
    python3 "$SCRIPT_DIR/review.py" local
    ;;
  staged)
    echo -e "${YELLOW}🔍 审查已暂存的变更...${NC}"
    python3 "$SCRIPT_DIR/review.py" staged
    ;;
  pr)
    BASE="${TARGET:-main}"
    echo -e "${YELLOW}🔍 审查当前分支与 ${BASE} 的差异...${NC}"
    python3 "$SCRIPT_DIR/review.py" pr "$BASE"
    ;;
  file)
    if [ -z "$TARGET" ]; then
      echo "Usage: $0 file <path>"
      exit 1
    fi
    echo -e "${YELLOW}🔍 审查文件: ${TARGET}${NC}"
    python3 "$SCRIPT_DIR/review.py" file "$TARGET"
    ;;
  diff)
    if [ -z "$TARGET" ]; then
      echo "Usage: $0 diff <diff-file>"
      exit 1
    fi
    echo -e "${YELLOW}🔍 审查 diff 文件: ${TARGET}${NC}"
    python3 "$SCRIPT_DIR/review.py" diff "$TARGET"
    ;;
  help|--help|-h)
    echo "Kimi Code Review - 本地代码审查工具（LiteMark 定制版）"
    echo ""
    echo "用法:"
    echo "  $0 local              审查未提交的变更 (git diff)"
    echo "  $0 staged             审查已暂存的变更 (git diff --staged)"
    echo "  $0 pr [base_branch]   审查当前分支与 base 的差异 (默认 main)"
    echo "  $0 file <path>        审查单个文件"
    echo "  $0 diff <file>        审查已有的 diff 文件"
    echo ""
    echo "环境变量:"
    echo "  KIMI_AGENT_FILE       Agent 文件路径"
    echo "  KIMI_MODEL            指定模型"
    echo "  REVIEW_MAX_DIFF       diff 最大行数 (默认 800)"
    echo "  REVIEW_OUTPUT         报告输出文件路径"
    echo "  REVIEW_JSON_OUTPUT    JSON 输出路径 (默认 review_result.json)"
    ;;
  *)
    echo "未知模式: $MODE"
    echo "Run '$0 help' for usage."
    exit 1
    ;;
esac
