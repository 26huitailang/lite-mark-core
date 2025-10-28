# LiteMark 使用示例

## 基本用法

### 单张图片处理

```bash
# 使用经典模板为单张图片添加水印
litemark add -i photo.jpg -t classic -o photo_watermarked.jpg

# 使用现代模板
litemark add -i photo.jpg -t modern -o photo_watermarked.jpg

# 使用极简模板
litemark add -i photo.jpg -t minimal -o photo_watermarked.jpg

# 自定义作者名
litemark add -i photo.jpg -t classic -o photo_watermarked.jpg --author "John Doe"
```

### 批量处理

```bash
# 批量处理整个目录
litemark batch -i /path/to/photos -t classic -o /path/to/output

# 使用现代模板批量处理
litemark batch -i photos/ -t modern -o watermarked_photos/

# 批量处理并设置统一作者名
litemark batch -i photos/ -t classic -o output/ --author "Photographer Name"
```

### 模板管理

```bash
# 查看所有可用模板
litemark templates

# 查看特定模板的详细信息
litemark show-template classic
litemark show-template modern
litemark show-template minimal
```

## 模板自定义

### 创建自定义模板

1. 使用 `litemark show-template classic > my_template.json` 导出模板
2. 编辑 JSON 文件自定义样式
3. 使用 `litemark add -i photo.jpg -t my_template.json -o output.jpg`

### 模板变量

支持的变量：
- `{Author}` - 摄影师姓名
- `{ISO}` - ISO 感光度
- `{Aperture}` - 光圈值
- `{Shutter}` - 快门速度
- `{Focal}` - 焦距
- `{Camera}` - 相机型号
- `{Lens}` - 镜头型号
- `{DateTime}` - 拍摄时间

### 模板示例

#### 经典模板 (ClassicParam)
- 位置：左下角
- 内容：摄影师姓名 + 拍摄参数
- 背景：半透明黑色矩形

#### 现代模板 (Modern)
- 位置：右上角
- 内容：相机型号 + 镜头 + 拍摄参数
- 背景：半透明黑色矩形

#### 极简模板 (Minimal)
- 位置：右下角
- 内容：仅摄影师姓名
- 背景：无

## 高级用法

### 处理不同格式

```bash
# JPEG 图片
litemark add -i photo.jpg -t classic -o output.jpg

# PNG 图片
litemark add -i photo.png -t modern -o output.png

# 批量处理多种格式
litemark batch -i photos/ -t classic -o output/
```

### 性能优化

- 对于大量图片，建议使用批量处理
- 大图片处理可能需要更多时间
- 建议在 SSD 上运行以获得更好性能

## 故障排除

### 常见问题

1. **模板未找到**
   - 检查模板名称是否正确
   - 使用 `litemark templates` 查看可用模板

2. **图片无法加载**
   - 检查文件路径是否正确
   - 确保图片格式受支持（JPEG, PNG）

3. **输出文件无法保存**
   - 检查输出目录是否存在
   - 确保有写入权限

### 获取帮助

```bash
# 查看帮助信息
litemark --help

# 查看特定命令帮助
litemark add --help
litemark batch --help
```
