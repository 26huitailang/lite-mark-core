import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { EditorMode, TemplateConfig } from '@/types'
import { PRESETS } from '@/utils/templatePresets'

export const useTemplateStore = defineStore('template', () => {
  const editorMode = ref<EditorMode>('preset')
  const selectedPreset = ref<string>('classic')
  const customTemplate = ref<TemplateConfig>(JSON.parse(JSON.stringify(PRESETS.classic)))
  const jsonText = ref<string>(JSON.stringify(PRESETS.classic, null, 2))
  const jsonError = ref<string | null>(null)
  const lastValidTemplate = ref<TemplateConfig>(JSON.parse(JSON.stringify(PRESETS.classic)))

  const effectiveTemplateJson = computed<string>(() => {
    if (editorMode.value === 'preset') {
      return JSON.stringify(PRESETS[selectedPreset.value] ?? PRESETS.classic)
    }
    if (editorMode.value === 'json') {
      try {
        const parsed = JSON.parse(jsonText.value) as TemplateConfig
        jsonError.value = null
        lastValidTemplate.value = parsed
        return JSON.stringify(parsed)
      } catch (e) {
        jsonError.value = e instanceof Error ? e.message : 'Invalid JSON'
        return JSON.stringify(lastValidTemplate.value)
      }
    }
    // visual mode
    return JSON.stringify(customTemplate.value)
  })

  function selectPreset(presetId: string): void {
    selectedPreset.value = presetId
    const preset = PRESETS[presetId]
    if (preset) {
      customTemplate.value = JSON.parse(JSON.stringify(preset))
      jsonText.value = JSON.stringify(preset, null, 2)
      lastValidTemplate.value = JSON.parse(JSON.stringify(preset))
      jsonError.value = null
    }
  }

  function updateVisualTemplate(partial: Partial<TemplateConfig>): void {
    customTemplate.value = { ...customTemplate.value, ...partial }
    jsonText.value = JSON.stringify(customTemplate.value, null, 2)
  }

  function updateJsonText(text: string): void {
    jsonText.value = text
    try {
      const parsed = JSON.parse(text) as TemplateConfig
      jsonError.value = null
      lastValidTemplate.value = parsed
      customTemplate.value = JSON.parse(JSON.stringify(parsed))
    } catch (e) {
      jsonError.value = e instanceof Error ? e.message : 'Invalid JSON'
    }
  }

  function setEditorMode(mode: EditorMode): void {
    editorMode.value = mode
  }

  return {
    editorMode,
    selectedPreset,
    customTemplate,
    jsonText,
    jsonError,
    lastValidTemplate,
    effectiveTemplateJson,
    selectPreset,
    updateVisualTemplate,
    updateJsonText,
    setEditorMode,
  }
})
