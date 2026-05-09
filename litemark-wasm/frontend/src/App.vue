<template>
  <div class="app">
    <header class="app-header">
      <h1>📸 LiteMark</h1>
      <p class="subtitle">在浏览器本地为照片添加 EXIF 参数水印</p>
    </header>

    <main class="app-main">
      <section class="panel upload-panel">
        <h2 class="panel-title">1. 上传图片</h2>
        <UploadArea />
        <FileList />
      </section>

      <section class="panel template-panel">
        <h2 class="panel-title">2. 选择模板</h2>
        <TemplateEditor />
      </section>

      <section class="panel controls-panel">
        <h2 class="panel-title">3. 处理设置</h2>
        <ProcessingControls ref="controlsRef" @process="handleProcess" />
        <ProgressBar />
      </section>

      <ResultGallery />
    </main>

    <MessageToast ref="toastRef" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useFilesStore } from '@/stores/files'
import { useTemplateStore } from '@/stores/template'
import { useProcessingStore } from '@/stores/processing'
import { useSettingsStore } from '@/stores/settings'
import { useWasm } from '@/composables/useWasm'
import { parseWasmError } from '@/types'
import UploadArea from './components/UploadArea.vue'
import FileList from './components/FileList.vue'
import TemplateEditor from './components/TemplateEditor/TemplateEditor.vue'
import ProcessingControls from './components/ProcessingControls.vue'
import ProgressBar from './components/ProgressBar.vue'
import ResultGallery from './components/ResultGallery.vue'
import MessageToast from './components/MessageToast.vue'

const filesStore = useFilesStore()
const templateStore = useTemplateStore()
const processingStore = useProcessingStore()
const settingsStore = useSettingsStore()
const wasm = useWasm()

const controlsRef = ref<InstanceType<typeof ProcessingControls>>()
const toastRef = ref<InstanceType<typeof MessageToast>>()

onMounted(async () => {
  // Initialize WASM
  try {
    await wasm.initialize()
    processingStore.setStatus('ready')
    toastRef.value?.success('WASM 模块加载成功')
  } catch {
    processingStore.setStatus('error')
    toastRef.value?.error('WASM 加载失败，请刷新页面重试')
  }

  // Restore last preset
  if (settingsStore.lastPreset && templateStore.selectedPreset !== settingsStore.lastPreset) {
    templateStore.selectPreset(settingsStore.lastPreset)
  }
})

async function handleProcess() {
  if (!filesStore.hasFiles || !wasm.isReady.value) return

  processingStore.clearResults()
  processingStore.setStatus('processing')
  processingStore.resetCancel()

  const author = controlsRef.value?.author || undefined
  const templateJson = templateStore.effectiveTemplateJson
  const startTime = Date.now()

  try {
    const imageBytesArray = await Promise.all(
      filesStore.selectedFiles.map(async (f) => {
        const buffer = await f.file.arrayBuffer()
        return new Uint8Array(buffer)
      })
    )

    let logoBytes: Uint8Array | undefined
    if (filesStore.logoFile) {
      const buffer = await filesStore.logoFile.arrayBuffer()
      logoBytes = new Uint8Array(buffer)
    }

    const onProgress = (completed: number, total: number) => {
      processingStore.updateProgress(completed, total)
    }

    const results = wasm.process_batch(
      imageBytesArray,
      templateJson,
      author,
      undefined,
      logoBytes,
      onProgress
    )

    const elapsed = Date.now() - startTime
    processingStore.processingTimeMs = elapsed

    for (let i = 0; i < results.length; i++) {
      if (processingStore.cancelRequested.value) {
        toastRef.value?.info(`已取消，处理了 ${i} 张图片`)
        break
      }

      const outputBytes = results[i]
      const blob = new Blob([outputBytes], { type: 'image/jpeg' })
      const originalFile = filesStore.selectedFiles[i]

      processingStore.addResult({
        id: `${Date.now()}-${i}`,
        originalName: originalFile.originalName,
        outputBytes,
        outputUrl: URL.createObjectURL(blob),
        originalUrl: URL.createObjectURL(originalFile.file),
        outputSize: outputBytes.length,
        elapsedMs: 0,
      })
    }

    if (!processingStore.cancelRequested.value) {
      toastRef.value?.success(`✅ 完成 ${results.length} 张图片，耗时 ${(elapsed / 1000).toFixed(2)}s`)
    }

    if (author) {
      settingsStore.addAuthorToHistory(author)
    }
    settingsStore.lastPreset = templateStore.selectedPreset
    settingsStore.lastEditorMode = templateStore.editorMode
  } catch (err) {
    const wasmErr = parseWasmError(err)
    processingStore.setWasmError(wasmErr.message)
    toastRef.value?.error(`处理失败: ${wasmErr.message}`)
    console.error(wasmErr)
  } finally {
    processingStore.setStatus('idle')
  }
}
</script>

<style>
* {
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  margin: 0;
  padding: 0;
  background: #f5f5f5;
  color: #333;
  line-height: 1.5;
}

.app {
  max-width: 900px;
  margin: 0 auto;
  padding: 20px;
}

.app-header {
  text-align: center;
  margin-bottom: 24px;
}

.app-header h1 {
  margin: 0 0 6px;
  font-size: 28px;
  font-weight: 600;
  color: #1a1a1a;
}

.subtitle {
  margin: 0;
  color: #999;
  font-size: 14px;
}

.app-main {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.panel {
  background: white;
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
}

.panel-title {
  margin: 0 0 14px;
  font-size: 15px;
  font-weight: 600;
  color: #333;
  padding-bottom: 10px;
  border-bottom: 1px solid #f0f0f0;
}

@media (max-width: 640px) {
  .app {
    padding: 12px;
  }

  .panel {
    padding: 14px;
  }
}
</style>
