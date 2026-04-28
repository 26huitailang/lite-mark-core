#!/usr/bin/env python3
"""
文档扫描脚本：扫描项目中的所有文档，生成待确认清单
"""

import json
import os
import sys
from datetime import datetime, timezone
from pathlib import Path


# 排除的目录（agent 过程文档和构建输出）
EXCLUDED_DIRS = {
    '.git',
    '.kimi',
    '.trae',
    'archive',
    'target',
    'node_modules',
    'venv',
    '.venv',
    '__pycache__',
}


# 文档文件扩展名
DOC_EXTENSIONS = {'.md', '.mdx', '.rst', '.txt'}


# 配置文件也视为文档
CONFIG_DOCS = {
    'Cargo.toml',
    'package.json',
    'pyproject.toml',
    'setup.py',
    'setup.cfg',
    'requirements.txt',
    'Makefile',
}


def is_excluded(path: Path, root: Path) -> bool:
    """检查路径是否应被排除"""
    # 检查是否在不排除的目录中
    for part in path.relative_to(root).parts:
        if part in EXCLUDED_DIRS:
            return True
    return False


def get_doc_type(file_path: Path) -> str:
    """判断文档类型"""
    name = file_path.name.lower()
    
    if name == 'readme.md':
        return 'readme'
    elif name == 'agents.md':
        return 'agents'
    elif name == 'architecture.md':
        return 'architecture'
    elif name == 'changelog.md' or name.startswith('changelog'):
        return 'changelog'
    elif name in ('todo.md', 'todo'):
        return 'todo'
    elif name == 'plan.md':
        return 'plan'
    elif name == 'skills.md':
        return 'skills'
    elif file_path.suffix in ('.json', '.toml', '.yaml', '.yml'):
        return 'config'
    else:
        return 'documentation'


def get_priority(doc_type: str) -> str:
    """获取文档优先级"""
    priorities = {
        'readme': 'high',
        'agents': 'high',
        'architecture': 'high',
        'skills': 'high',
        'config': 'medium',
        'changelog': 'medium',
        'documentation': 'medium',
        'todo': 'low',
        'plan': 'low',
    }
    return priorities.get(doc_type, 'medium')


def get_check_items(doc_type: str) -> list:
    """获取该文档类型的检查项"""
    items = {
        'readme': [
            "项目描述是否准确",
            "安装步骤是否可用",
            "快速开始示例是否正确",
            "功能列表是否完整",
        ],
        'agents': [
            "项目结构描述是否与目录一致",
            "技术栈版本是否正确",
            "构建命令是否可用",
            "开发规范是否被遵循",
        ],
        'architecture': [
            "架构描述是否与代码一致",
            "模块依赖关系是否正确",
            "数据流描述是否准确",
        ],
        'skills': [
            "skill 描述是否准确",
            "触发条件是否合理",
        ],
        'config': [
            "配置项说明是否完整",
            "默认值是否正确",
            "示例配置是否可用",
        ],
        'changelog': [
            "版本号是否正确",
            "变更描述是否准确",
            "日期是否正确",
        ],
        'documentation': [
            "内容是否过时",
            "链接是否有效",
            "示例代码是否正确",
        ],
        'todo': [
            "待办事项是否已完成",
            "优先级是否合理",
        ],
        'plan': [
            "计划步骤是否完成",
            "时间安排是否合理",
        ],
    }
    return items.get(doc_type, ["内容是否准确", "是否过时"])


def scan_docs(root: Path) -> dict:
    """扫描文档"""
    files = []
    
    for path in root.rglob('*'):
        if not path.is_file():
            continue
        
        if is_excluded(path, root):
            continue
        
        # 检查是否是文档文件
        is_doc = (
            path.suffix.lower() in DOC_EXTENSIONS or
            path.name in CONFIG_DOCS or
            (path.suffix.lower() in ('.json', '.toml', '.yaml', '.yml') and 
             ('template' in path.name.lower() or 'config' in path.name.lower()))
        )
        
        if not is_doc:
            continue
        
        doc_type = get_doc_type(path)
        rel_path = str(path.relative_to(root))
        
        files.append({
            'path': rel_path,
            'type': doc_type,
            'priority': get_priority(doc_type),
            'check_items': get_check_items(doc_type),
        })
    
    # 按优先级排序
    priority_order = {'high': 0, 'medium': 1, 'low': 2}
    files.sort(key=lambda x: priority_order.get(x['priority'], 1))
    
    return {
        'scan_time': datetime.now(timezone.utc).isoformat(),
        'total_files': len(files),
        'files': files,
    }


def main():
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path.cwd()
    result = scan_docs(root)
    
    output_file = root / '.kimi' / 'doc_scan_result.json'
    output_file.parent.mkdir(parents=True, exist_ok=True)
    
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(result, f, ensure_ascii=False, indent=2)
    
    print(f"扫描完成，共发现 {result['total_files']} 个文档")
    print(f"结果已保存至: {output_file}")
    print(f"优先级分布:")
    for p in ['high', 'medium', 'low']:
        count = sum(1 for f in result['files'] if f['priority'] == p)
        print(f"  - {p}: {count} 个文件")


if __name__ == '__main__':
    main()
