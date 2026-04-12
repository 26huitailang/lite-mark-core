# 使用说明

## 当前状态

✅ **步骤1完成**: 清理了代码警告
✅ **步骤2进行中**: wasm-pack 正在下载依赖（需要等待完成）
✅ **步骤3完成**: HTML测试页面已创建

## wasm-pack 构建说明

wasm-pack 正在后台运行，下载 wasm-bindgen 工具。完成后会在 `pkg/` 目录生成以下文件：
- `litemark_wasm.js` - JavaScript绑定
- `litemark_wasm_bg.wasm` - WASM二进制
- `litemark_wasm.d.ts` - TypeScript类型定义
- `package.json` - npm包配置

## 使用方法

### 方法1：等待wasm-pack完成（推荐）
等待后台进程完成，然后：
```bash
cd litemark-wasm
python3 -m http.server 8000
# 或
npx serve .
```
访问 http://localhost:8000/demo.html

### 方法2：手动构建（如果后台进程卡住）
```bash
# 取消后台进程
# 重新运行
wasm-pack build litemark-wasm --target web --release
```

## 测试页面功能

`demo.html` 提供以下功能：
- ✅ 拖拽或点击上传多张图片
- ✅ 自定义作者名称
- ✅ 自定义模板JSON
- ✅ 批量处理进度显示
- ✅ 实时预览与下载

## API 导出

WASM模块导出三个函数：
1. `process_image(imageBytes, templateJson, author, fontBytes, logoBytes)` - 完整单图处理
2. `process_image_basic(imageBytes, templateJson)` - 简化单图处理
3. `process_batch(images, templateJson, author, fontBytes, logoBytes, onProgress)` - 批量处理

## 注意事项

- 不支持 HEIC/HEIF 格式（已在wasm32目标禁用）
- 支持 JPG、PNG、WebP 等常见格式
- 使用默认嵌入字体（思源黑体）
- 所有处理均在浏览器本地完成，无需上传
