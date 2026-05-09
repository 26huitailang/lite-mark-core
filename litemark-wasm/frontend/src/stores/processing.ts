import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { ProcessResult } from '@/types'

export type ProcessingStatus = 'idle' | 'initializing' | 'ready' | 'processing' | 'error'

export const useProcessingStore = defineStore('processing', () => {
  const status = ref<ProcessingStatus>('idle')
  const wasmError = ref<string | null>(null)
  const progress = ref(0)
  const currentIndex = ref(0)
  const totalCount = ref(0)
  const results = ref<ProcessResult[]>([])
  const cancelRequested = ref(false)
  const processingTimeMs = ref(0)

  const isProcessing = computed(() => status.value === 'processing')
  const isReady = computed(() => status.value === 'ready')
  const hasResults = computed(() => results.value.length > 0)

  function setStatus(newStatus: ProcessingStatus): void {
    status.value = newStatus
  }

  function setWasmError(error: string | null): void {
    wasmError.value = error
    if (error) status.value = 'error'
  }

  function updateProgress(completed: number, total: number): void {
    currentIndex.value = completed
    totalCount.value = total
    progress.value = total > 0 ? Math.round((completed / total) * 100) : 0
  }

  function addResult(result: ProcessResult): void {
    results.value.push(result)
  }

  function clearResults(): void {
    results.value = []
    progress.value = 0
    currentIndex.value = 0
    totalCount.value = 0
    processingTimeMs.value = 0
  }

  function requestCancel(): void {
    cancelRequested.value = true
  }

  function resetCancel(): void {
    cancelRequested.value = false
  }

  return {
    status,
    wasmError,
    progress,
    currentIndex,
    totalCount,
    results,
    cancelRequested,
    processingTimeMs,
    isProcessing,
    isReady,
    hasResults,
    setStatus,
    setWasmError,
    updateProgress,
    addResult,
    clearResults,
    requestCancel,
    resetCancel,
  }
})
