<template>
  <div class="visual-mode">
    <div class="form-columns">
      <div class="form-column">
        <div class="field-group">
          <label>模板名称</label>
          <input v-model="template.name" type="text" class="text-input" />
        </div>

        <div class="field-group">
          <label>锚点位置</label>
          <select v-model="template.anchor" class="select-input">
            <option v-for="a in anchors" :key="a" :value="a">{{ a }}</option>
          </select>
        </div>

        <div class="field-group">
          <label>渲染模式</label>
          <select v-model="template.render_mode" class="select-input">
            <option v-for="m in renderModes" :key="m" :value="m">{{ m }}</option>
          </select>
        </div>

        <div class="field-group">
          <label>边框高度比例: {{ template.frame_height_ratio }}</label>
          <input v-model.number="template.frame_height_ratio" type="range" min="0.02" max="0.25" step="0.01" class="range-input" />
        </div>

        <div class="field-group">
          <label>Logo 大小比例: {{ template.logo_size_ratio }}</label>
          <input v-model.number="template.logo_size_ratio" type="range" min="0" max="0.8" step="0.01" class="range-input" />
        </div>
      </div>

      <div class="form-column">
        <div class="field-group">
          <label>主字体比例: {{ template.primary_font_ratio }}</label>
          <input v-model.number="template.primary_font_ratio" type="range" min="0.1" max="1" step="0.01" class="range-input" />
        </div>

        <div class="field-group">
          <label>次字体比例: {{ template.secondary_font_ratio }}</label>
          <input v-model.number="template.secondary_font_ratio" type="range" min="0.05" max="0.8" step="0.01" class="range-input" />
        </div>

        <div class="field-group">
          <label>内边距比例: {{ template.padding_ratio }}</label>
          <input v-model.number="template.padding_ratio" type="range" min="0" max="0.5" step="0.01" class="range-input" />
        </div>

        <div class="field-group">
          <label>行间距比例: {{ template.line_spacing_ratio ?? 0.3 }}</label>
          <input v-model.number="template.line_spacing_ratio" type="range" min="0" max="1" step="0.05" class="range-input" />
        </div>
      </div>
    </div>

    <div class="items-section">
      <div class="items-header">
        <label>内容项</label>
        <div class="item-actions">
          <button class="add-btn" @click="addItem('text')">+ 文字</button>
          <button class="add-btn" @click="addItem('logo')">+ Logo</button>
        </div>
      </div>

      <div class="item-list">
        <div v-for="(item, index) in template.items" :key="index" class="item-row">
          <span class="item-type">{{ item.type === 'logo' ? 'Logo' : '文字' }}</span>
          <input v-if="item.type === 'text'" v-model="item.value" type="text" class="item-input" placeholder="输入内容，可用 {Author} 等变量" />
          <input v-else type="text" disabled value="(Logo 占位)" class="item-input disabled" />

          <select v-model="item.weight" class="item-select">
            <option :value="null">默认</option>
            <option value="normal">Normal</option>
            <option value="bold">Bold</option>
            <option value="light">Light</option>
          </select>

          <input v-model="item.color" type="color" class="item-color" />

          <button class="remove-item" @click="removeItem(index)">✕</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { watch } from 'vue'
import { useTemplateStore } from '@/stores/template'
import type { Anchor, RenderMode, ItemType } from '@/types'

const templateStore = useTemplateStore()

const template = templateStore.customTemplate

const anchors: Anchor[] = ['top-left', 'top-right', 'bottom-left', 'bottom-right', 'bottom-center', 'center']
const renderModes: RenderMode[] = ['bottom-frame', 'gradient-frame', 'overlay', 'minimal']

watch(() => template, () => {
  templateStore.updateVisualTemplate(template)
}, { deep: true })

function addItem(type: ItemType) {
  template.items.push({
    type,
    value: type === 'text' ? '' : '',
    font_size_ratio: 0.2,
    weight: 'normal',
    color: type === 'text' ? '#1A1A1A' : null,
  })
}

function removeItem(index: number) {
  template.items.splice(index, 1)
}
</script>

<style scoped>
.visual-mode {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.form-columns {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

@media (max-width: 600px) {
  .form-columns {
    grid-template-columns: 1fr;
  }
}

.form-column {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.field-group label {
  display: block;
  font-size: 12px;
  color: #666;
  margin-bottom: 4px;
}

.text-input,
.select-input {
  width: 100%;
  padding: 8px 10px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 13px;
  box-sizing: border-box;
}

.range-input {
  width: 100%;
  accent-color: #007AFF;
}

.items-section {
  border-top: 1px solid #e2e8f0;
  padding-top: 12px;
}

.items-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.items-header label {
  font-size: 13px;
  font-weight: 500;
  color: #333;
}

.item-actions {
  display: flex;
  gap: 8px;
}

.add-btn {
  padding: 4px 10px;
  border: 1px solid #007AFF;
  background: #eff6ff;
  color: #007AFF;
  border-radius: 6px;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.add-btn:hover {
  background: #007AFF;
  color: white;
}

.item-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.item-row {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  background: #f8fafc;
  border-radius: 6px;
}

.item-type {
  font-size: 11px;
  color: #999;
  width: 36px;
  flex-shrink: 0;
}

.item-input {
  flex: 1;
  padding: 6px 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 12px;
  min-width: 0;
}

.item-input.disabled {
  background: #f1f5f9;
  color: #999;
}

.item-select {
  width: 80px;
  padding: 6px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 12px;
  flex-shrink: 0;
}

.item-color {
  width: 32px;
  height: 28px;
  padding: 0;
  border: 1px solid #ddd;
  border-radius: 4px;
  flex-shrink: 0;
  cursor: pointer;
}

.remove-item {
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  color: #ccc;
  cursor: pointer;
  font-size: 12px;
  flex-shrink: 0;
}

.remove-item:hover {
  color: #ef4444;
}
</style>
