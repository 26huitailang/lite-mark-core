# LiteMark Frontend

Vue3 + Vite + TypeScript + Pinia 重构的 LiteMark WASM 演示页面。

## 开发

```bash
# 从项目根目录
make frontend-dev
```

或手动：

```bash
wasm-pack build litemark-wasm --target web --release
cd litemark-wasm/frontend
npm install
npm run dev
```

## 构建

```bash
make frontend-build
```

或手动：

```bash
wasm-pack build litemark-wasm --target web --release
cd litemark-wasm/frontend
npm ci
npm run build
```

构建输出在 `dist/` 目录。

## 部署

推送到 `main` 分支会自动触发 GitHub Actions 部署到 GitHub Pages。

## 项目结构

```
src/
├── components/          # Vue 组件
│   ├── UploadArea.vue   # 拖拽上传
│   ├── FileList.vue     # 文件列表
│   ├── TemplateEditor/  # 模板编辑器（三模式）
│   ├── ProcessingControls.vue  # 处理控制
│   ├── ProgressBar.vue  # 进度条
│   ├── ResultGallery.vue # 结果网格
│   ├── ResultCard.vue   # 单张结果卡片
│   ├── ImageCompare.vue # 前后对比
│   └── MessageToast.vue # 全局通知
├── stores/              # Pinia 状态管理
│   ├── files.ts
│   ├── template.ts
│   ├── processing.ts
│   └── settings.ts
├── composables/         # 组合式函数
│   ├── useWasm.ts
│   ├── useHeic.ts
│   └── useLocalStorage.ts
├── types/               # TypeScript 类型
├── utils/               # 工具函数
└── assets/presets/      # 内置模板 JSON
```
