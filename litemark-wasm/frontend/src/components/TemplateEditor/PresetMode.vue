<template>
  <div class="preset-mode">
    <label class="field-label">选择预设模板</label>
    <select v-model="selected" class="preset-select" @change="onChange">
      <option v-for="id in PRESET_IDS" :key="id" :value="id">
        {{ PRESET_LABELS[id] }}
      </option>
    </select>

    <div class="preset-preview">
      <div class="preview-label">预览效果</div>
      <div class="preview-card">
        <div class="preview-name">{{ currentPreset?.name }}</div>
        <div class="preview-mode">
          模式: <span class="mode-badge">{{ currentPreset?.render_mode }}</span>
        </div>
        <div class="preview-items">
          <span v-for="(item, i) in currentPreset?.items" :key="i" class="item-tag">
            {{ item.type === 'logo' ? 'Logo' : item.value.substring(0, 20) }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useTemplateStore } from '@/stores/template'
import { PRESETS, PRESET_IDS, PRESET_LABELS } from '@/utils/templatePresets'

const templateStore = useTemplateStore()

const selected = computed({
  get: () => templateStore.selectedPreset,
  set: (val) => templateStore.selectPreset(val),
})

const currentPreset = computed(() => PRESETS[templateStore.selectedPreset])

function onChange() {
  // handled by computed setter
}
</script>

<style scoped>
.preset-mode {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.field-label {
  font-size: 13px;
  font-weight: 500;
  color: #333;
}

.preset-select {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 14px;
  background: white;
}

.preset-preview {
  margin-top: 4px;
}

.preview-label {
  font-size: 12px;
  color: #999;
  margin-bottom: 6px;
}

.preview-card {
  padding: 12px;
  background: #f8fafc;
  border-radius: 8px;
  border: 1px solid #e2e8f0;
}

.preview-name {
  font-weight: 600;
  color: #333;
  margin-bottom: 6px;
}

.preview-mode {
  font-size: 12px;
  color: #666;
  margin-bottom: 8px;
}

.mode-badge {
  padding: 2px 8px;
  background: #eff6ff;
  color: #007AFF;
  border-radius: 4px;
  font-size: 11px;
}

.preview-items {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.item-tag {
  padding: 2px 8px;
  background: #f1f5f9;
  color: #64748b;
  font-size: 11px;
  border-radius: 4px;
}
</style>
