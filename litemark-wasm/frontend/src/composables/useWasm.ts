import { ref } from 'vue'
import { parseWasmError } from '@/types'
import init, { process_batch } from '@wasm/litemark_wasm.js'

export function useWasm() {
  const isReady = ref(false)
  const error = ref<string | null>(null)
  const isInitializing = ref(false)

  async function initialize(): Promise<void> {
    if (isReady.value) return
    isInitializing.value = true
    error.value = null
    try {
      await init()
      isReady.value = true
    } catch (err) {
      const wasmErr = parseWasmError(err)
      error.value = wasmErr.message
      throw wasmErr
    } finally {
      isInitializing.value = false
    }
  }

  return {
    isReady,
    error,
    isInitializing,
    initialize,
    process_batch,
  }
}
