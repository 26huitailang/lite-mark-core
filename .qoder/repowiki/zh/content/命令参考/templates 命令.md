# templates 命令

<cite>
**本文档中引用的文件**
- [src/main.rs](file://src/main.rs)
- [src/layout/mod.rs](file://src/layout/mod.rs)
- [templates/classic.json](file://templates/classic.json)
- [templates/minimal.json](file://templates/minimal.json)
- [templates/modern.json](file://templates/modern.json)
- [Cargo.toml](file://Cargo.toml)
- [README.md](file://README.md)
</cite>

## 概述

`templates` 命令是 lite-mark-core 工具的核心功能之一，用于列出系统支持的所有内置模板。该命令无需任何额外参数，在执行后会显示 `templates/` 目录下所有可用的水印模板名称及其简要描述。

## 命令语法

```bash
litemark templates
```

## 功能特性

### 无参数设计
`templates` 命令采用极简设计理念，不接受任何参数。这种设计确保了命令的易用性和一致性，用户只需输入 `litemark templates` 即可获取完整的信息。

### 自动发现机制
命令通过调用 `list_templates()` 函数自动扫描并列出所有可用模板，无需手动配置或维护模板列表。

### 实时信息更新
每次执行命令都会重新读取模板目录，确保显示的信息始终是最新的。

## 输出格式

执行 `litemark templates` 后，命令将输出以下格式的信息：

```
Available templates:
  • ClassicParam - Bottom-left corner with photographer name and basic parameters
  • Modern - Top-right corner with clean typography
  • Minimal - Subtle bottom-right signature
```

### 输出结构说明

| 字段 | 描述 | 示例值 |
|------|------|--------|
| 模板名称 | 内置模板的正式名称 | `ClassicParam`, `Modern`, `Minimal` |
| 简要描述 | 模板风格和布局特点的简短说明 | `Bottom-left corner with photographer name and basic parameters` |

## 模板机制概述

lite-mark-core 使用基于 JSON 的模板系统，每个模板定义了水印的布局、样式和内容。模板系统的核心组件包括：

### 模板结构
每个模板包含以下核心属性：
- **name**: 模板标识符
- **anchor**: 锚点位置（如 `bottom-left`）
- **padding**: 内边距设置
- **items**: 显示元素列表
- **background**: 背景样式配置

### 变量替换系统
模板支持动态变量替换，如 `{Author}`, `{ISO}`, `{Aperture}` 等，这些变量会在渲染时被实际的 EXIF 数据替换。

## 模板类型详解

### ClassicParam 模式
- **锚点**: 底部左下角
- **特点**: 经典的摄影参数展示方式
- **内容**: 包含摄影师姓名和基本拍摄参数
- **适用场景**: 需要完整参数信息的摄影作品

### Modern 模式  
- **锚点**: 顶部右上角
- **特点**: 现代简约风格
- **内容**: 设备信息和高级参数组合
- **适用场景**: 追求现代感和简洁设计的作品

### Minimal 模式
- **锚点**: 底部右下角
- **特点**: 极简签名风格
- **内容**: 仅显示摄影师姓名
- **适用场景**: 需要低调签名效果的作品

## 使用示例

### 基本使用
```bash
# 列出所有可用模板
litemark templates
```

### 结合其他命令使用
```bash
# 查看模板详情
litemark show-template classic

# 使用特定模板处理图片
litemark add -i input.jpg -t classic -o output.jpg

# 批量处理使用模板
litemark batch -i ./photos/ -t modern -o ./processed/
```

## 模板文件管理

### 文件位置
模板文件位于项目的 `templates/` 目录下，包含三个默认模板：
- `classic.json`: 经典参数模板
- `minimal.json`: 极简签名模板  
- `modern.json`: 现代风格模板

### 命名规则
- 文件名即模板名称（小写）
- JSON 格式，包含完整的模板定义
- 支持自定义模板扩展

### 扩展机制
用户可以通过添加自定义 JSON 文件来扩展模板列表：
1. 在 `templates/` 目录创建新模板文件
2. 文件名将成为模板名称
3. 命令会自动识别并列出新模板

## API 调用参考

对于希望集成模板管理功能的开发者，以下是相关 API 的调用方式：

### 获取模板列表
```rust
// 调用内置模板创建函数
let templates = litemark::layout::create_builtin_templates();

// 遍历模板信息
for template in templates {
    println!("{} - {}", template.name, describe_template(&template));
}
```

### 模板描述函数
```rust
fn describe_template(template: &Template) -> &'static str {
    match template.name.as_str() {
        "ClassicParam" => "Bottom-left corner with photographer name and basic parameters",
        "Modern" => "Top-right corner with clean typography",
        "Minimal" => "Subtle bottom-right signature",
        _ => "Custom template",
    }
}
```

## 错误处理

### 模板不可用
如果系统无法找到指定的模板，会返回相应的错误信息。虽然 `templates` 命令本身不会失败，但建议在程序中处理可能的异常情况。

### 权限问题
确保程序对 `templates/` 目录具有读取权限，否则可能导致模板加载失败。

## 性能考虑

### 加载效率
模板列表的加载是即时操作，性能开销极小，适合频繁调用。

### 内存占用
模板数据在内存中的占用很小，不会对系统资源造成显著影响。

## 最佳实践

### 模板选择指南
1. **ClassicParam**: 适用于需要完整参数记录的场景
2. **Modern**: 适用于追求现代设计感的作品
3. **Minimal**: 适用于需要低调签名效果的场景

### 自定义模板开发
- 建议从现有模板开始修改
- 注意保持 JSON 格式的正确性
- 测试自定义模板的兼容性

## 相关命令

### show-template 命令
配合 `templates` 命令使用，可以查看单个模板的详细配置：
```bash
litemark show-template classic
```

### add 和 batch 命令
使用 `templates` 命令发现的模板名称作为参数：
```bash
litemark add -i input.jpg -t ClassicParam -o output.jpg
litemark batch -i ./photos/ -t Modern -o ./output/
```

**节来源**
- [src/main.rs](file://src/main.rs#L100-L105)
- [src/layout/mod.rs](file://src/layout/mod.rs#L100-L205)
- [templates/classic.json](file://templates/classic.json#L1-L27)
- [templates/modern.json](file://templates/modern.json#L1-L29)
- [templates/minimal.json](file://templates/minimal.json#L1-L17)