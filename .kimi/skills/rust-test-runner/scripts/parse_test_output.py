#!/usr/bin/env python3
"""
Parse cargo test output and extract error information.
"""

import sys
import re
import json


def parse_cargo_test_output(output):
    """Parse cargo test output and extract errors."""
    errors = []
    warnings = []
    
    lines = output.split('\n')
    
    # Pattern for error location: "  --> file:line:col"
    location_pattern = re.compile(r'^\s+-->\s+([^:]+):(\d+):(\d+)')
    # Pattern for error header: "error: message" or "error[E0000]: message"
    error_pattern = re.compile(r'^error(?:\[(E\d+)\])?:\s*(.+)$')
    # Pattern for warning: "warning: message"
    warning_pattern = re.compile(r'^warning:\s*(.+)$')
    # Pattern for help message
    help_pattern = re.compile(r'^\s+help:\s*(.+)$')
    # Pattern for note
    note_pattern = re.compile(r'^\s+= note:\s*(.+)$')
    
    i = 0
    while i < len(lines):
        line = lines[i]
        
        # Check for error header
        error_match = error_pattern.match(line)
        if error_match:
            error_code = error_match.group(1)
            error_msg = error_match.group(2)
            
            # Look ahead for location
            file_path = None
            line_num = None
            col_num = None
            help_msg = None
            note_msg = None
            context_lines = [line]
            
            for j in range(i+1, min(len(lines), i+20)):
                context_lines.append(lines[j])
                
                loc_match = location_pattern.match(lines[j])
                if loc_match:
                    file_path = loc_match.group(1)
                    line_num = int(loc_match.group(2))
                    col_num = int(loc_match.group(3))
                
                help_match = help_pattern.match(lines[j])
                if help_match:
                    help_msg = help_match.group(1)
                
                note_match = note_pattern.match(lines[j])
                if note_match:
                    note_msg = note_match.group(1)
                
                # Stop at next error/warning or empty line after context
                if j > i + 3 and lines[j].strip() == '' and file_path:
                    break
            
            error_info = {
                'file': file_path or 'unknown',
                'line': line_num or 0,
                'column': col_num or 0,
                'message': error_msg,
                'error_code': error_code,
                'help': help_msg,
                'note': note_msg,
                'full_context': '\n'.join(context_lines)
            }
            errors.append(error_info)
        
        # Check for warning header
        warning_match = warning_pattern.match(line)
        if warning_match:
            warning_msg = warning_match.group(1)
            
            # Look ahead for location
            file_path = None
            line_num = None
            col_num = None
            help_msg = None
            context_lines = [line]
            
            for j in range(i+1, min(len(lines), i+20)):
                context_lines.append(lines[j])
                
                loc_match = location_pattern.match(lines[j])
                if loc_match:
                    file_path = loc_match.group(1)
                    line_num = int(loc_match.group(2))
                    col_num = int(loc_match.group(3))
                
                help_match = help_pattern.match(lines[j])
                if help_match:
                    help_msg = help_match.group(1)
                
                if j > i + 3 and lines[j].strip() == '' and file_path:
                    break
            
            warning_info = {
                'file': file_path or 'unknown',
                'line': line_num or 0,
                'column': col_num or 0,
                'message': warning_msg,
                'help': help_msg,
                'full_context': '\n'.join(context_lines)
            }
            warnings.append(warning_info)
        
        i += 1
    
    return {'errors': errors, 'warnings': warnings}


def categorize_error(error):
    """Categorize error and provide fix suggestion."""
    msg = error['message'].lower()
    full = error.get('full_context', '').lower()
    
    # String concatenation in println
    if "expected `,`, found `+`" in msg or ("found `+`" in full and 'println!' in full):
        return {
            'category': 'string_concatenation',
            'description': 'Rust 不支持用 + 拼接字符串字面量',
            'fix_suggestion': '使用 format!("\\n{}", "=".repeat(50)) 或 println!("\\n{}", "=".repeat(50))',
            'auto_fixable': True,
            'priority': 'high'
        }
    
    # Duplicate import
    if 'defined multiple times' in msg or 'reimported' in msg:
        return {
            'category': 'duplicate_import',
            'description': '重复导入同名类型或模块',
            'fix_suggestion': '删除重复的 import 语句，保留一个即可',
            'auto_fixable': True,
            'priority': 'high'
        }
    
    # Missing derive macro
    if 'cannot find derive macro' in msg:
        macro_name = msg.split("'")[1] if "'" in msg else "Unknown"
        return {
            'category': 'missing_derive',
            'description': f'缺少 {macro_name} derive macro',
            'fix_suggestion': f'确保 Cargo.toml 中添加了 serde = {{ version = "1.0", features = ["derive"] }} 并在代码中使用 use serde::{macro_name};',
            'auto_fixable': False,
            'priority': 'medium'
        }
    
    return {
        'category': 'unknown',
        'description': error['message'],
        'fix_suggestion': '需要手动检查代码',
        'auto_fixable': False,
        'priority': 'low'
    }


def categorize_warning(warning):
    """Categorize warning and provide fix suggestion."""
    msg = warning['message'].lower()
    
    if 'unused import' in msg:
        return {
            'category': 'unused_import',
            'description': '未使用的 import',
            'fix_suggestion': '删除未使用的 import，或如果是有意保留，添加 #[allow(unused_imports)]',
            'auto_fixable': True,
            'priority': 'low'
        }
    
    if 'unused variable' in msg:
        return {
            'category': 'unused_variable',
            'description': '未使用的变量',
            'fix_suggestion': '变量名前加下划线前缀，如 _result，或删除该变量',
            'auto_fixable': True,
            'priority': 'low'
        }
    
    if 'never used' in msg or 'dead_code' in msg:
        return {
            'category': 'dead_code',
            'description': '从未使用的代码（函数/结构体等）',
            'fix_suggestion': '删除未使用的代码，或添加 #[allow(dead_code)]',
            'auto_fixable': True,
            'priority': 'low'
        }
    
    return {
        'category': 'unknown_warning',
        'description': warning['message'],
        'fix_suggestion': '根据 warning 提示处理',
        'auto_fixable': False,
        'priority': 'low'
    }


def main():
    output = sys.stdin.read()
    result = parse_cargo_test_output(output)
    
    # Categorize
    for error in result['errors']:
        error['category_info'] = categorize_error(error)
    
    for warning in result['warnings']:
        warning['category_info'] = categorize_warning(warning)
    
    # Print summary
    print(f"=== 解析结果 ===")
    print(f"错误数: {len(result['errors'])}")
    print(f"警告数: {len(result['warnings'])}")
    print()
    
    if result['errors']:
        print("=== 错误详情 ===")
        for i, err in enumerate(result['errors'], 1):
            cat = err['category_info']
            print(f"\n{i}. {err['file']}:{err['line']}")
            print(f"   类别: {cat['category']}")
            print(f"   问题: {cat['description']}")
            if err.get('error_code'):
                print(f"   错误码: {err['error_code']}")
            print(f"   原消息: {err['message']}")
            print(f"   建议修复: {cat['fix_suggestion']}")
            print(f"   优先级: {cat['priority']}")
            print(f"   可自动修复: {'是' if cat['auto_fixable'] else '否'}")
    
    if result['warnings']:
        print("\n=== 警告详情 ===")
        for i, warn in enumerate(result['warnings'], 1):
            cat = warn['category_info']
            print(f"\n{i}. {warn['file']}:{warn['line']}")
            print(f"   类别: {cat['category']}")
            print(f"   问题: {cat['description']}")
            print(f"   建议修复: {cat['fix_suggestion']}")
    
    # Output JSON for programmatic use
    print("\n=== JSON 输出 ===")
    print(json.dumps(result, indent=2, ensure_ascii=False))


if __name__ == '__main__':
    main()
