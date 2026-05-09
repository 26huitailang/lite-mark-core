<template>
  <div class="upload-section">
    <div
      class="upload-area"
      :class="{ dragover: isDragOver }"
      @click="triggerFileInput"
      @dragover.prevent="isDragOver = true"
      @dragleave.prevent="isDragOver = false"
      @drop.prevent="handleDrop"
    >
      <input
        ref="fileInput"
        type="file"
        accept="image/*"
        multiple
        class="file-input"
        @change="handleFileSelect"
      />
      <div class="upload-content">
        <div class="upload-icon">📸</div>
        <p class="upload-title">
          <strong>点击选择图片</strong> 或 <strong>拖拽图片到此处</strong>
        </p>
        <p class="upload-hint">支持 JPG、PNG、WebP 等格式</p>
        <p v-if="heicSupported === false" class="upload-warning">
          ⚠️ HEIC/HEIF 格式仅 Safari 支持，其他浏览器请先转换
        </p>
      </div>
    </div>

    <div v-if="filesStore.rejectedFiles.length > 0" class="rejected-banner">
      <p class="rejected-title">⚠️ 以下文件已自动过滤：</p>
      <ul class="rejected-list">
        <li v-for="f in filesStore.rejectedFiles" :key="f.name">
          {{ f.name }} —
          <span v-if="f.reason === 'heic-unsupported'">浏览器不支持 HEIC</span>
          <span v-else-if="f.reason === 'conversion-failed'">HEIC 转换失败</span>
          <span v-else>不支持的格式</span>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useFilesStore } from '@/stores/files'
import { useHeic } from '@/composables/useHeic'

const filesStore = useFilesStore()
const { heicSupported, checkSupport } = useHeic()

const fileInput = ref<HTMLInputElement>()
const isDragOver = ref(false)

checkSupport()

function triggerFileInput() {
  fileInput.value?.click()
}

async function handleFileSelect(e: Event) {
  const input = e.target as HTMLInputElement
  if (!input.files?.length) return
  await filesStore.addFiles(Array.from(input.files))
  input.value = ''
}

async function handleDrop(e: DragEvent) {
  isDragOver.value = false
  const files = Array.from(e.dataTransfer?.files || [])
    .filter(f => f.type.startsWith('image/') || /\.(heic|heif)$/i.test(f.name))
  if (files.length) await filesStore.addFiles(files)
}
</script>

<style scoped>
.upload-section {
  margin-bottom: 16px;
}

.upload-area {
  border: 2px dashed #ddd;
  border-radius: 12px;
  padding: 40px;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s;
  background: #fafafa;
}

.upload-area:hover,
.upload-area.dragover {
  border-color: #007AFF;
  background: #f0f7ff;
}

.file-input {
  display: none;
}

.upload-icon {
  font-size: 48px;
  margin-bottom: 12px;
}

.upload-title {
  margin: 0 0 8px;
  color: #333;
  font-size: 15px;
}

.upload-hint {
  margin: 0;
  color: #999;
  font-size: 13px;
}

.upload-warning {
  margin: 8px 0 0;
  color: #ff9800;
  font-size: 12px;
}

.rejected-banner {
  margin-top: 12px;
  padding: 12px 16px;
  background: #fff3e0;
  border-radius: 8px;
  border-left: 3px solid #ff9800;
}

.rejected-title {
  margin: 0 0 8px;
  font-weight: 500;
  color: #e65100;
  font-size: 13px;
}

.rejected-list {
  margin: 0;
  padding-left: 16px;
  font-size: 12px;
  color: #666;
}

.rejected-list li {
  margin-bottom: 4px;
}
</style>
