<template>
  <div class="template-editor">
    <div class="mode-tabs">
      <button
        v-for="mode in modes"
        :key="mode.key"
        class="mode-tab"
        :class="{ active: templateStore.editorMode === mode.key }"
        @click="setMode(mode.key)"
      >
        {{ mode.label }}
      </button>
    </div>

    <div class="mode-content">
      <PresetMode v-if="templateStore.editorMode === 'preset'" />
      <VisualMode v-else-if="templateStore.editorMode === 'visual'" />
      <JsonMode v-else />
    </div>

    <div v-if="templateStore.jsonError" class="json-error">
      ⚠️ {{ templateStore.jsonError }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { useTemplateStore } from '@/stores/template'
import type { EditorMode } from '@/types'
import PresetMode from './PresetMode.vue'
import VisualMode from './VisualMode.vue'
import JsonMode from './JsonMode.vue'

const templateStore = useTemplateStore()

const modes: { key: EditorMode; label: string }[] = [
  { key: 'preset', label: '预设' },
  { key: 'visual', label: '可视化' },
  { key: 'json', label: 'JSON' },
]

function setMode(mode: EditorMode) {
  templateStore.setEditorMode(mode)
}
</script>

<style scoped>
.template-editor {
  margin-bottom: 16px;
  border: 1px solid #e2e8f0;
  border-radius: 12px;
  overflow: hidden;
}

.mode-tabs {
  display: flex;
  border-bottom: 1px solid #e2e8f0;
}

.mode-tab {
  flex: 1;
  padding: 10px;
  border: none;
  background: #f8fafc;
  color: #64748b;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.mode-tab:hover {
  background: #f1f5f9;
}

.mode-tab.active {
  background: white;
  color: #007AFF;
  font-weight: 500;
  border-bottom: 2px solid #007AFF;
  margin-bottom: -1px;
}

.mode-content {
  padding: 16px;
}

.json-error {
  padding: 10px 16px;
  background: #fef3c7;
  color: #b45309;
  font-size: 12px;
  border-top: 1px solid #fde68a;
}
</style>
