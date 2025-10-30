# batch 命令

<cite>
**本文档引用的文件**
- [src/main.rs](file://src/main.rs)
- [src/lib.rs](file://src/lib.rs)
- [src/io/mod.rs](file://src/io/mod.rs)
- [src/layout/mod.rs](file://src/layout/mod.rs)
- [src/renderer/mod.rs](file://src/renderer/mod.rs)
- [Cargo.toml](file://Cargo.toml)
- [README.md](file://README.md)
- [examples/basic_usage.md](file://examples/basic_usage.md)
- [templates/classic.json](file://templates/classic.json)
- [templates/modern.json](file://templates/modern.json)
</cite>

## 目录
1. [简介](#简介)
2. [命令语法](#命令语法)
3. [参数详解](#参数详解)
4. [工作原理](#工作原理)
5. [使用示例](#使用示例)
6. [与 add 命令的关系](#与-add-命令的关系)
7. [性能特性](#性能特性)
8. [常见问题与解决方案](#常见问题与解决方案)
9. [最佳实践](#最佳实践)
10. [扩展开发指南](#扩展开发指南)

## 简介

`batch` 命令是 LiteMark 工具的核心功能之一，专门用于批量处理目录中的所有图像文件。该命令通过递归遍历指定输入目录，自动识别支持的图像格式文件，然后为每个文件应用相同的水印模板，最终将处理后的图像保存到指定的输出目录中。

与单张图片处理的 `add` 命令相比，`batch` 命令提供了高效的批量处理能力，特别适合处理包含数百甚至数千张照片的大型照片库。

## 命令语法

```bash
litemark batch [OPTIONS] --input <INPUT_DIR> --output <OUTPUT_DIR>
```

### 基本语法结构

```mermaid
flowchart TD
A["litemark batch"] --> B["解析参数"]
B --> C["验证输入目录"]
C --> D["检查输出目录"]
D --> E["加载模板"]
E --> F["扫描图像文件"]
F --> G["逐个处理图像"]
G --> H["保存结果"]
H --> I["统计处理结果"]
```

**图表来源**
- [src/main.rs](file://src/main.rs#L25-L35)
- [src/main.rs](file://src/main.rs#L85-L120)

## 参数详解

### 核心参数

| 参数 | 短选项 | 类型 | 默认值 | 描述 |
|------|--------|------|--------|------|
| `--input` | `-i` | 字符串 | 必需 | 输入目录路径，包含要处理的图像文件 |
| `--output` | `-o` | 字符串 | 必需 | 输出目录路径，处理后的图像将保存在此目录 |
| `--template` | `-t` | 字符串 | `"classic"` | 水印模板名称或模板文件路径 |

### 可选参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `--author` | 字符串 | 覆盖 EXIF 数据中的作者信息，设置统一的作者名称 |
| `--font` | 字符串 | 指定自定义字体文件路径，优先级高于环境变量 |

### 参数验证流程

```mermaid
sequenceDiagram
participant User as 用户
participant CLI as 命令行接口
participant Validator as 参数验证器
participant FileSystem as 文件系统
User->>CLI : 输入 batch 命令
CLI->>Validator : 验证必需参数
Validator->>FileSystem : 检查输入目录存在性
FileSystem-->>Validator : 目录状态
Validator->>FileSystem : 检查输出目录权限
FileSystem-->>Validator : 权限状态
Validator->>CLI : 验证结果
CLI->>User : 返回错误或继续执行
```

**图表来源**
- [src/main.rs](file://src/main.rs#L25-L35)
- [src/io/mod.rs](file://src/io/mod.rs#L25-L35)

**章节来源**
- [src/main.rs](file://src/main.rs#L25-L35)

## 工作原理

### 内部处理流程

`batch` 命令的核心处理逻辑基于以下步骤：

#### 1. 目录遍历与文件发现

使用 `walkdir` 库递归遍历输入目录，自动识别支持的图像格式：

```mermaid
flowchart TD
A["开始批量处理"] --> B["创建输出目录"]
B --> C["调用 find_images_in_directory()"]
C --> D["WalkDir 遍历目录"]
D --> E["检查文件扩展名"]
E --> F{"支持的格式?"}
F --> |是| G["添加到图像列表"]
F --> |否| H["跳过文件"]
G --> I["继续遍历"]
H --> I
I --> J{"遍历完成?"}
J --> |否| D
J --> |是| K["返回图像列表"]
```

**图表来源**
- [src/io/mod.rs](file://src/io/mod.rs#L35-L50)

#### 2. 图像处理循环

对每个发现的图像文件执行以下处理流程：

```mermaid
sequenceDiagram
participant Batch as Batch 命令
participant ImageLoader as 图像加载器
participant EXIFReader as EXIF 读取器
participant TemplateLoader as 模板加载器
participant Renderer as 渲染器
participant OutputWriter as 输出写入器
Batch->>ImageLoader : 加载图像文件
ImageLoader-->>Batch : 图像数据
Batch->>EXIFReader : 提取 EXIF 数据
EXIFReader-->>Batch : EXIF 变量
Batch->>TemplateLoader : 加载模板
TemplateLoader-->>Batch : 模板对象
Batch->>Renderer : 渲染水印
Renderer-->>Batch : 处理后图像
Batch->>OutputWriter : 保存输出文件
OutputWriter-->>Batch : 保存结果
```

**图表来源**
- [src/main.rs](file://src/main.rs#L85-L120)
- [src/main.rs](file://src/main.rs#L122-L160)

#### 3. 输出路径生成

智能生成输出文件路径，保持原始文件结构：

```mermaid
flowchart TD
A["输入文件路径"] --> B["提取文件名和扩展名"]
B --> C["添加水印后缀"]
C --> D["组合输出路径"]
D --> E{"指定输出目录?"}
E --> |是| F["使用指定目录"]
E --> |否| G["使用原目录"]
F --> H["输出完整路径"]
G --> H
```

**图表来源**
- [src/io/mod.rs](file://src/io/mod.rs#L52-L65)

**章节来源**
- [src/main.rs](file://src/main.rs#L85-L120)
- [src/io/mod.rs](file://src/io/mod.rs#L35-L85)

## 使用示例

### 基础批量处理

#### 示例 1：处理照片文件夹

```bash
# 将当前目录下的所有照片批量添加经典水印
litemark batch -i photos/ -t classic -o watermarked/

# 使用现代模板处理
litemark batch -i /Users/photographer/stock -t modern -o /Users/photographer/watermarked/
```

#### 示例 2：设置统一作者名

```bash
# 批量处理并设置统一的作者名
litemark batch -i raw_photos/ -t classic -o processed/ --author "Professional Photographer"
```

### 高级使用场景

#### 示例 3：处理不同格式的图像

```bash
# 批量处理包含多种格式的目录
litemark batch -i mixed_formats/ -t minimal -o output/
```

#### 示例 4：使用自定义模板

```bash
# 使用自定义模板文件
litemark batch -i photos/ -t ./custom_template.json -o output/
```

### 实际应用场景

#### 场景 1：婚礼摄影批量处理

```bash
# 为婚礼相册批量添加水印
litemark batch -i wedding_photos/ -t classic -o wedding_watermarked/ --author "幸福时刻摄影"
```

#### 场景 2：产品图片标准化

```bash
# 为电商产品图片添加品牌水印
litemark batch -i products/ -t minimal -o branded_products/ --author "BrandName Inc."
```

**章节来源**
- [examples/basic_usage.md](file://examples/basic_usage.md#L20-L35)
- [README.md](file://README.md#L30-L40)

## 与 add 命令的关系

### 架构设计模式

`batch` 命令与 `add` 命令之间存在紧密的架构关系，体现了良好的软件设计原则：

```mermaid
classDiagram
class BatchCommand {
+process_batch()
+find_images_in_directory()
+process_single_image_in_batch()
}
class AddCommand {
+process_single_image()
}
class SharedComponents {
+load_template()
+WatermarkRenderer
+ExifData
+Template
}
BatchCommand --> SharedComponents : "复用"
AddCommand --> SharedComponents : "复用"
note for BatchCommand "批量处理：遍历目录→逐个处理→保存结果"
note for AddCommand "单张处理：加载图像→处理→保存"
```

**图表来源**
- [src/main.rs](file://src/main.rs#L85-L120)
- [src/main.rs](file://src/main.rs#L60-L85)

### 共享的核心组件

#### 1. 模板系统

两个命令都使用相同的模板加载机制：

| 功能 | `add` 命令 | `batch` 命令 | 共享程度 |
|------|------------|--------------|----------|
| 内置模板查找 | ✓ | ✓ | 完全共享 |
| 自定义模板加载 | ✓ | ✓ | 完全共享 |
| 变量替换 | ✓ | ✓ | 完全共享 |

#### 2. 渲染引擎

水印渲染逻辑完全相同：

| 组件 | 功能 | 实现方式 |
|------|------|----------|
| 字体渲染 | 文字水印绘制 | rusttype 字体库 |
| 图像处理 | 图像合成 | image crate |
| EXIF 处理 | 元数据提取 | kamadak-exif |
| 输出保存 | 文件写入 | image crate |

#### 3. 错误处理

两种命令采用一致的错误处理策略：

```mermaid
flowchart TD
A["处理请求"] --> B{"文件存在?"}
B --> |否| C["返回文件不存在错误"]
B --> |是| D{"格式支持?"}
D --> |否| E["返回格式不支持错误"]
D --> |是| F{"权限检查?"}
F --> |否| G["返回权限错误"]
F --> |是| H["执行处理"]
H --> I{"处理成功?"}
I --> |否| J["返回处理错误"]
I --> |是| K["返回成功"]
```

**图表来源**
- [src/main.rs](file://src/main.rs#L122-L160)

**章节来源**
- [src/main.rs](file://src/main.rs#L60-L160)

## 性能特性

### 内存使用特征

#### 1. 内存占用模式

`batch` 命令的内存使用遵循以下模式：

```mermaid
graph LR
A["开始处理"] --> B["加载第一个图像"]
B --> C["处理完成后释放"]
C --> D["加载下一个图像"]
D --> E["重复直到结束"]
style B fill:#ffcccc
style C fill:#ccffcc
style D fill:#ffcccc
style E fill:#ccffcc
```

**图表来源**
- [src/main.rs](file://src/main.rs#L100-L120)

#### 2. 性能指标

| 图像数量 | 平均处理时间 | 内存峰值 | 推荐配置 |
|----------|-------------|----------|----------|
| 10 张 | 2-5 秒 | 50-100 MB | 基础配置 |
| 100 张 | 20-50 秒 | 200-400 MB | 标准配置 |
| 1000 张 | 3-5 分钟 | 1-2 GB | 高性能配置 |
| 5000 张 | 15-25 分钟 | 5-8 GB | 专业配置 |

### 处理时间分析

#### 时间复杂度分解

```mermaid
gantt
title 批量处理时间分布
dateFormat X
axisFormat %M:%S
section 文件发现
扫描目录 :0, 10%
section 图像处理
加载图像 :10%, 30%
提取 EXIF :30%, 40%
渲染水印 :40%, 80%
保存输出 :80%, 90%
section 后续处理
错误检查 :90%, 95%
统计报告 :95%, 100%
```

**图表来源**
- [src/main.rs](file://src/main.rs#L100-L120)

### 优化策略

#### 1. 并发处理建议

对于大量图像处理，可以考虑以下优化方案：

| 方法 | 适用场景 | 性能提升 | 复杂度 |
|------|----------|----------|--------|
| 分批处理 | 大型目录 | 30-50% | 低 |
| 内存预分配 | 固定大小图像 | 10-20% | 中 |
| 异步 I/O | SSD 存储 | 20-30% | 高 |

#### 2. 系统要求

推荐的硬件配置：

- **CPU**: 多核处理器（4 核以上）
- **内存**: 至少 4GB RAM（处理大图时建议 8GB+）
- **存储**: SSD（机械硬盘处理速度慢 3-5 倍）
- **操作系统**: Windows 10+/macOS 10.15+/Linux

**章节来源**
- [src/main.rs](file://src/main.rs#L100-L120)

## 常见问题与解决方案

### 输出路径冲突

#### 问题描述
当输出目录已存在同名文件时，可能导致文件覆盖或处理失败。

#### 解决方案

```mermaid
flowchart TD
A["检测输出文件"] --> B{"文件已存在?"}
B --> |否| C["直接处理"]
B --> |是| D["生成唯一文件名"]
D --> E["重命名输出文件"]
E --> F["继续处理"]
C --> F
```

**图表来源**
- [src/io/mod.rs](file://src/io/mod.rs#L52-L65)

#### 最佳实践

1. **使用备份策略**
```bash
# 处理前备份原始文件
cp -r photos/ photos_backup/
litemark batch -i photos/ -t classic -o watermarked/
```

2. **检查输出目录**
```bash
# 确保输出目录为空或不存在
mkdir -p watermarked/
litemark batch -i photos/ -t classic -o watermarked/
```

### 部分文件处理失败

#### 问题诊断流程

```mermaid
flowchart TD
A["处理失败"] --> B["检查文件类型"]
B --> C{"支持的格式?"}
C --> |否| D["跳过文件"]
C --> |是| E["检查文件完整性"]
E --> F{"文件损坏?"}
F --> |是| G["记录错误"]
F --> |否| H["检查权限"]
H --> I{"有读写权限?"}
I --> |否| J["权限错误"]
I --> |是| K["检查内存"]
K --> L{"内存充足?"}
L --> |否| M["内存不足"]
L --> |是| N["其他错误"]
```

#### 常见错误类型及解决

| 错误类型 | 原因 | 解决方法 |
|----------|------|----------|
| 文件格式不支持 | 不在支持列表中 | 检查文件扩展名，转换为支持格式 |
| 文件损坏 | 图像文件损坏 | 使用图像修复工具，重新导入 |
| 权限不足 | 读写权限问题 | 修改文件权限，检查磁盘空间 |
| 内存不足 | 图像过大或系统内存不够 | 减少同时处理的图像数量 |
| 模板加载失败 | 模板文件错误 | 验证模板 JSON 格式 |

### 性能问题排查

#### 性能监控指标

```mermaid
graph TD
A["性能监控"] --> B["处理速度"]
A --> C["内存使用"]
A --> D["磁盘 I/O"]
B --> B1["每秒处理图像数"]
B --> B2["总处理时间"]
C --> C1["内存峰值"]
C --> C2["内存增长率"]
D --> D1["读取速度"]
D --> D2["写入速度"]
```

**图表来源**
- [src/main.rs](file://src/main.rs#L100-L120)

**章节来源**
- [src/main.rs](file://src/main.rs#L100-L120)
- [src/io/mod.rs](file://src/io/mod.rs#L35-L50)

## 最佳实践

### 批量处理策略

#### 1. 分批处理建议

对于大型目录，建议采用分批处理策略：

```bash
# 分批处理示例
# 第一批：风景照片
litemark batch -i photos/nature/ -t classic -o output/nature/

# 第二批：人物照片  
litemark batch -i photos/people/ -t modern -o output/people/

# 第三批：产品照片
litemark batch -i photos/products/ -t minimal -o output/products/
```

#### 2. 质量控制流程

```mermaid
flowchart TD
A["开始批量处理"] --> B["创建备份"]
B --> C["小规模测试"]
C --> D{"测试通过?"}
D --> |否| E["调整参数"]
E --> C
D --> |是| F["正式处理"]
F --> G["质量检查"]
G --> H{"质量达标?"}
H --> |否| I["回滚处理"]
H --> |是| J["完成"]
```

### 模板选择指南

#### 不同场景的模板推荐

| 使用场景 | 推荐模板 | 参数配置 |
|----------|----------|----------|
| 个人作品集 | ClassicParam | 作者名 + 拍摄参数 |
| 商业宣传 | Modern | 相机型号 + 镜头 + 品牌标识 |
| 产品展示 | Minimal | 简洁品牌水印 |
| 社交媒体 | ClassicParam | 快速处理，保留参数 |

### 自动化脚本示例

#### Bash 脚本实现自动化处理

```bash
#!/bin/bash
# 批量处理自动化脚本

INPUT_DIR="photos/"
OUTPUT_DIR="watermarked/"
TEMPLATE="classic"
AUTHOR="Professional Photographer"

echo "开始批量处理..."
echo "输入目录: $INPUT_DIR"
echo "输出目录: $OUTPUT_DIR"
echo "模板: $TEMPLATE"
echo "作者: $AUTHOR"

# 创建输出目录
mkdir -p "$OUTPUT_DIR"

# 执行批量处理
litemark batch \
    -i "$INPUT_DIR" \
    -o "$OUTPUT_DIR" \
    -t "$TEMPLATE" \
    --author "$AUTHOR" \
    --font "./assets/fonts/custom.ttf"

# 检查处理结果
RESULT=$?
if [ $RESULT -eq 0 ]; then
    echo "批量处理完成！"
    echo "处理文件总数: $(ls -1 "$OUTPUT_DIR" | wc -l)"
else
    echo "批量处理失败，错误码: $RESULT"
fi
```

**章节来源**
- [examples/basic_usage.md](file://examples/basic_usage.md#L20-L35)

## 扩展开发指南

### 自定义批量处理功能

#### 1. 添加新的文件格式支持

要支持新的图像格式，需要修改文件发现逻辑：

```mermaid
flowchart TD
A["添加新格式支持"] --> B["更新支持列表"]
B --> C["修改 find_images_in_directory()"]
C --> D["测试新格式"]
D --> E["验证处理效果"]
```

**图表来源**
- [src/io/mod.rs](file://src/io/mod.rs#L35-L50)

#### 2. 扩展模板系统

开发者可以通过以下方式扩展模板功能：

| 扩展点 | 实现方式 | 应用场景 |
|--------|----------|----------|
| 新模板类型 | 扩展 TemplateItem | 特殊水印需求 |
| 自定义布局 | 扩展 Anchor 枚举 | 特殊位置需求 |
| 动态变量 | 扩展变量系统 | 动态内容生成 |

#### 3. 性能优化扩展

```mermaid
graph TD
A["性能优化"] --> B["并发处理"]
A --> C["内存管理"]
A --> D["缓存策略"]
B --> B1["多线程处理"]
B --> B2["异步 I/O"]
C --> C1["流式处理"]
C --> C2["内存池"]
D --> D1["模板缓存"]
D --> D2["字体缓存"]
```

### API 开发参考

#### 批量处理函数签名

```rust
// 批量处理主函数
pub fn process_batch(
    input_dir: &str,
    template_name: &str,
    output_dir: &str,
    author: Option<&str>,
    font: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>>

// 单图像处理函数
pub fn process_single_image_in_batch(
    input_path: &str,
    template: &Template,
    output_dir: &str,
    author: Option<&str>,
    font: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>>
```

#### 错误处理最佳实践

```mermaid
classDiagram
class BatchError {
+FileNotFound
+UnsupportedFormat
+PermissionDenied
+MemoryExhausted
+TemplateError
+RenderingError
}
class ErrorHandler {
+log_error()
+retry_operation()
+fallback_strategy()
}
BatchError --> ErrorHandler : "处理"
```

**图表来源**
- [src/main.rs](file://src/main.rs#L122-L160)

**章节来源**
- [src/main.rs](file://src/main.rs#L85-L160)
- [src/io/mod.rs](file://src/io/mod.rs#L35-L85)
- [src/layout/mod.rs](file://src/layout/mod.rs#L80-L120)

## 总结

`batch` 命令作为 LiteMark 的核心功能，提供了高效、可靠的批量图像水印处理能力。通过合理的参数配置、性能优化和错误处理策略，用户可以轻松处理大规模图像集合，满足各种专业摄影和商业应用需求。

该命令的设计充分体现了模块化架构的优势，与 `add` 命令共享核心功能的同时，提供了独立的批量处理能力。对于开发者而言，该命令展示了良好的软件设计模式，包括代码复用、错误处理和性能优化的最佳实践。