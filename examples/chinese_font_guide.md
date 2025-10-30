# 中文字体配置指南

LiteMark 默认字体（DejaVu Sans）仅支持英文字符。如需在水印中显示中文摄影师姓名或其他中文文本，请按以下方式配置中文字体。

## 快速开始

### 方法 1：使用命令行参数（推荐）

```bash
litemark add \
  -i photo.jpg \
  -o output.jpg \
  -t classic \
  --author "张三" \
  --font "/System/Library/Fonts/PingFang.ttc"
```

### 方法 2：设置环境变量

```bash
# 设置环境变量
export LITEMARK_FONT="/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc"

# 之后的所有命令都会使用此字体
litemark add -i photo.jpg -o output.jpg --author "李四"
```

## 推荐的中文字体

### macOS

| 字体名称 | 路径 | 文件大小 | 特点 |
|----------|------|----------|------|
| **PingFang SC** | `/System/Library/Fonts/PingFang.ttc` | 约 15MB | 系统默认，优雅现代 |
| **Heiti SC** | `/System/Library/Fonts/STHeiti Medium.ttc` | 约 10MB | 黑体，粗细适中 |
| **Songti SC** | `/System/Library/Fonts/Songti.ttc` | 约 20MB | 宋体，传统风格 |

**推荐使用：**
```bash
--font "/System/Library/Fonts/PingFang.ttc"
```

### Linux (Ubuntu/Debian)

需要先安装中文字体包：

```bash
# 安装 Noto Sans CJK（Google 开源字体）
sudo apt-get install fonts-noto-cjk

# 安装文泉驿字体
sudo apt-get install fonts-wqy-microhei fonts-wqy-zenhei
```

| 字体名称 | 路径 | 特点 |
|----------|------|------|
| **Noto Sans CJK SC** | `/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc` | Google 开源，质量高 |
| **WenQuanYi Micro Hei** | `/usr/share/fonts/truetype/wqy/wqy-microhei.ttc` | 文泉驿微米黑，轻量 |
| **WenQuanYi Zen Hei** | `/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc` | 文泉驿正黑，粗体 |

**推荐使用：**
```bash
--font "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc"
```

### Windows

| 字体名称 | 路径 | 特点 |
|----------|------|------|
| **Microsoft YaHei** | `C:\Windows\Fonts\msyh.ttc` | 微软雅黑，现代清晰 |
| **SimHei** | `C:\Windows\Fonts\simhei.ttf` | 黑体 |
| **SimSun** | `C:\Windows\Fonts\simsun.ttc` | 宋体 |

**推荐使用（PowerShell）：**
```powershell
litemark add -i photo.jpg -o output.jpg --font "C:\Windows\Fonts\msyh.ttc"
```

**推荐使用（CMD）：**
```cmd
litemark add -i photo.jpg -o output.jpg --font "C:\Windows\Fonts\msyh.ttc"
```

## 开源中文字体推荐

如果你需要在商业项目中使用，或者系统中没有预装中文字体，可以下载以下开源字体：

### 1. Noto Sans CJK（思源黑体）

- **授权：** SIL Open Font License 1.1（可商用）
- **下载：** https://github.com/googlefonts/noto-cjk/releases
- **特点：** Google 和 Adobe 合作开发，质量极高，支持中日韩

**下载和使用：**
```bash
# 下载（选择 SC 简体中文版本）
wget https://github.com/googlefonts/noto-cjk/releases/download/Sans2.004/NotoSansCJKsc.zip
unzip NotoSansCJKsc.zip
mkdir -p ~/.local/share/fonts
mv *.otf ~/.local/share/fonts/
fc-cache -fv  # Linux 刷新字体缓存

# 使用
litemark add -i photo.jpg -o output.jpg \
  --font "$HOME/.local/share/fonts/NotoSansCJKsc-Regular.otf"
```

### 2. 思源宋体（Noto Serif CJK）

- **授权：** SIL Open Font License 1.1
- **下载：** https://github.com/googlefonts/noto-cjk/releases
- **特点：** 宋体风格，适合传统中文排版

### 3. 文泉驿系列

- **授权：** GPLv2（可商用）
- **下载：** http://wenq.org/wqy2/
- **特点：** 中国开源字体项目，轻量级

### 4. Sarasa Gothic（更纱黑体）

- **授权：** SIL Open Font License 1.1
- **下载：** https://github.com/be5invis/Sarasa-Gothic/releases
- **特点：** 基于思源黑体改进，等宽版本适合代码

## 字体文件格式支持

LiteMark（rusttype）支持以下字体格式：

| 格式 | 扩展名 | 支持状态 | 说明 |
|------|--------|----------|------|
| TrueType | `.ttf` | ✅ 完全支持 | 最常见的字体格式 |
| OpenType | `.otf` | ✅ 支持 | 现代字体格式 |
| TrueType Collection | `.ttc` | ✅ 支持 | 多字体合集（如 PingFang.ttc） |

## 常见问题

### Q1: 为什么中文显示为方块（□□□）？

**原因：** 使用的字体不支持中文字符。

**解决方案：**
1. 检查是否正确指定了中文字体路径
2. 确认字体文件确实支持中文（打开字体文件查看）
3. 使用本指南推荐的中文字体

### Q2: 字体文件找不到怎么办？

**macOS：**
```bash
# 查找系统中所有 TrueType 字体
find /System/Library/Fonts -name "*.ttf" -o -name "*.ttc"
find /Library/Fonts -name "*.ttf" -o -name "*.ttc"
```

**Linux：**
```bash
# 查找已安装的字体
fc-list | grep -i chinese
fc-list :lang=zh
```

**Windows：**
```powershell
# PowerShell
Get-ChildItem C:\Windows\Fonts -Filter *.ttf
Get-ChildItem C:\Windows\Fonts -Filter *.ttc
```

### Q3: 可以同时支持中英文吗？

**可以！** 使用支持中文的字体（如 Noto Sans CJK）即可同时渲染中英文：

```bash
litemark add -i photo.jpg -o output.jpg \
  --font "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc" \
  --author "John Doe 摄影"
```

### Q4: 字体文件太大怎么办？

**选择轻量级字体：**
- WenQuanYi Micro Hei（约 5MB）
- Noto Sans CJK SC Regular（约 16MB）

**字体子集化（高级）：**
使用 `pyftsubset` 工具只保留需要的字符：

```bash
# 安装工具
pip install fonttools

# 提取常用汉字（简体中文基本字符集）
pyftsubset NotoSansCJKsc-Regular.otf \
  --text-file=common_chars.txt \
  --output-file=NotoSansCJKsc-Regular-subset.otf
```

### Q5: 环境变量在哪里设置？

**macOS/Linux (bash)：**
```bash
# 临时设置（当前终端会话）
export LITEMARK_FONT="/path/to/font.ttf"

# 永久设置（添加到 ~/.bashrc 或 ~/.zshrc）
echo 'export LITEMARK_FONT="/path/to/font.ttf"' >> ~/.bashrc
source ~/.bashrc
```

**Windows (PowerShell)：**
```powershell
# 临时设置
$env:LITEMARK_FONT = "C:\Windows\Fonts\msyh.ttc"

# 永久设置（系统环境变量）
[Environment]::SetEnvironmentVariable("LITEMARK_FONT", "C:\Windows\Fonts\msyh.ttc", "User")
```

## 批量处理示例

### 使用中文作者名批量处理照片

```bash
# 方法 1：命令行参数
litemark batch \
  -i ./photos/ \
  -o ./output/ \
  -t classic \
  --author "王摄影师" \
  --font "/System/Library/Fonts/PingFang.ttc"

# 方法 2：环境变量
export LITEMARK_FONT="/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc"
litemark batch -i ./photos/ -o ./output/ --author "李摄影"
```

## 字体授权注意事项

### 商业使用

如果你在商业项目中使用 LiteMark：

✅ **安全的开源字体：**
- Noto Sans/Serif CJK（SIL OFL）
- 文泉驿系列（GPLv2）
- Sarasa Gothic（SIL OFL）

⚠️ **需要许可的字体：**
- 部分商业字体（方正、汉仪等）需要购买授权
- 某些系统预装字体有使用限制

**建议：** 商业项目使用 Noto Sans CJK，完全免费且质量高。

### 开源项目

- SIL OFL 授权的字体可以自由嵌入开源软件
- GPLv2 字体可用于 GPL 项目
- 建议在 README 中注明使用的字体及其授权

## 进阶配置

### 为不同模板使用不同字体

虽然 LiteMark 当前不直接支持模板级字体配置，但可以通过脚本实现：

```bash
#!/bin/bash
# process_with_fonts.sh

# 现代模板用思源黑体
export LITEMARK_FONT="/path/to/NotoSansCJK-Regular.otf"
litemark add -i photo1.jpg -o out1.jpg -t modern

# 经典模板用宋体
export LITEMARK_FONT="/path/to/NotoSerifCJK-Regular.otf"
litemark add -i photo2.jpg -o out2.jpg -t classic
```

### 自动检测系统字体

```bash
#!/bin/bash
# auto_detect_font.sh

# 按优先级尝试查找中文字体
if [ -f "/System/Library/Fonts/PingFang.ttc" ]; then
    FONT="/System/Library/Fonts/PingFang.ttc"
elif [ -f "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc" ]; then
    FONT="/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc"
elif [ -f "C:/Windows/Fonts/msyh.ttc" ]; then
    FONT="C:/Windows/Fonts/msyh.ttc"
else
    echo "No Chinese font found"
    exit 1
fi

litemark add -i "$1" -o "$2" --font "$FONT" --author "摄影师"
```

## 相关资源

- [Google Fonts - Noto CJK](https://github.com/googlefonts/noto-cjk)
- [文泉驿字体](http://wenq.org/)
- [rusttype 文档](https://docs.rs/rusttype/)
- [字体授权指南](https://opensource.org/licenses/OFL-1.1)

## 反馈和建议

如果你在字体配置上遇到问题，或有其他中文字体推荐，欢迎：

- 提交 [GitHub Issue](https://github.com/26huitailang/lite-mark-core/issues)
- 参与 [GitHub Discussions](https://github.com/26huitailang/lite-mark-core/discussions)
