import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { EditorMode } from '@/types'
import { useLocalStorage } from '@/composables/useLocalStorage'

export const useSettingsStore = defineStore('settings', () => {
  const lastAuthor = useLocalStorage<string>('litemark:lastAuthor', '')
  const lastPreset = useLocalStorage<string>('litemark:lastPreset', 'classic')
  const lastEditorMode = useLocalStorage<EditorMode>('litemark:lastEditorMode', 'preset')
  const authorHistory = useLocalStorage<string[]>('litemark:authorHistory', [])

  function addAuthorToHistory(author: string): void {
    if (!author.trim()) return
    const history = authorHistory.value.filter(a => a !== author)
    history.unshift(author)
    authorHistory.value = history.slice(0, 10)
  }

  return {
    lastAuthor,
    lastPreset,
    lastEditorMode,
    authorHistory,
    addAuthorToHistory,
  }
})
