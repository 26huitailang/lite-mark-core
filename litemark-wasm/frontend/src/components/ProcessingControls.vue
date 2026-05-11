<template>
  <div class="controls-section">
    <div class="control-group">
      <label>作者名称（可选）</label>
      <div class="author-input-wrapper">
        <input
          v-model="author"
          type="text"
          placeholder="留空则使用 EXIF 中的作者信息"
          class="text-input"
        />
        <div v-if="settingsStore.authorHistory.length > 0" class="history-dropdown">
          <button
            v-for="name in settingsStore.authorHistory.slice(0, 5)"
            :key="name"
            class="history-item"
            @click="author = name"
          >
            {{ name }}
          </button>
        </div>
      </div>
    </div>

    <div class="control-group">
      <label>Logo 图片（可选）</label>
      <div class="logo-input">
        <input
          ref="logoInput"
          type="file"
          accept="image/png,image/jpeg,image/jpg"
          @change="handleLogoSelect"
        />
        <div v-if="logoPreview" class="logo-preview">
          <img :src="logoPreview" alt="Logo preview" />
          <button class="logo-remove" @click="clearLogo">✕</button>
        </div>
      </div>
      <p class="input-hint">推荐使用 PNG 透明背景，尺寸建议 200×200 以上</p>
    </div>

    <div class="action-buttons">
      <button
        class="btn-primary"
        :disabled="!canProcess"
        @click="$emit('process')"
      >
        🚀 处理图片
      </button>
      <button
        v-if="processingStore.isProcessing"
        class="btn-secondary"
        @click="processingStore.requestCancel()"
      >
        取消
      </button>
    </div>

    <div v-if="error" class="error-banner">
      ❌ WASM 加载失败: {{ error }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useFilesStore } from '@/stores/files'
import { useProcessingStore } from '@/stores/processing'
import { useSettingsStore } from '@/stores/settings'
import { useWasm } from '@/composables/useWasm'

const emit = defineEmits<{
  process: []
}>()

const filesStore = useFilesStore()
const processingStore = useProcessingStore()
const settingsStore = useSettingsStore()
const { isReady, error, isInitializing } = useWasm()

const author = ref(settingsStore.lastAuthor)
const logoInput = ref<HTMLInputElement>()
const logoPreview = ref<string | null>(null)

watch(author, (val) => {
  settingsStore.lastAuthor = val
})

const canProcess = computed(() =>
  isReady.value &&
  filesStore.hasFiles &&
  !processingStore.isProcessing
)

function handleLogoSelect(e: Event) {
  const input = e.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  filesStore.setLogo(file)
  logoPreview.value = URL.createObjectURL(file)
}

function clearLogo() {
  filesStore.setLogo(null)
  logoPreview.value = null
  if (logoInput.value) logoInput.value.value = ''
}

defineExpose({ author, logoPreview })
</script>

<style scoped>
.controls-section {
  margin-bottom: 16px;
}

.control-group {
  margin-bottom: 16px;
}

.control-group label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: #333;
  margin-bottom: 6px;
}

.text-input {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 14px;
  transition: border-color 0.2s;
  box-sizing: border-box;
}

.text-input:focus {
  outline: none;
  border-color: #007AFF;
}

.author-input-wrapper {
  position: relative;
}

.history-dropdown {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-top: 6px;
}

.history-item {
  padding: 3px 10px;
  border: 1px solid #e2e8f0;
  border-radius: 12px;
  background: #f8fafc;
  font-size: 12px;
  color: #64748b;
  cursor: pointer;
  transition: all 0.2s;
}

.history-item:hover {
  background: #eff6ff;
  border-color: #007AFF;
  color: #007AFF;
}

.logo-input input[type="file"] {
  width: 100%;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 13px;
  box-sizing: border-box;
}

.logo-preview {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 8px;
}

.logo-preview img {
  width: 48px;
  height: 48px;
  object-fit: contain;
  border-radius: 4px;
  background: #f8fafc;
  border: 1px solid #e2e8f0;
}

.logo-remove {
  width: 24px;
  height: 24px;
  border: none;
  background: #fee2e2;
  color: #ef4444;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}

.input-hint {
  font-size: 11px;
  color: #999;
  margin: 4px 0 0;
}

.action-buttons {
  display: flex;
  gap: 10px;
}

.btn-primary {
  flex: 1;
  padding: 12px 20px;
  border: none;
  border-radius: 8px;
  background: #007AFF;
  color: white;
  font-size: 15px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-primary:hover:not(:disabled) {
  background: #0056b3;
}

.btn-primary:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.btn-secondary {
  padding: 12px 16px;
  border: 1px solid #ddd;
  border-radius: 8px;
  background: white;
  color: #666;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary:hover {
  background: #f5f5f5;
}

.error-banner {
  margin-top: 12px;
  padding: 10px 12px;
  background: #fee2e2;
  color: #ef4444;
  border-radius: 8px;
  font-size: 13px;
}
</style>
