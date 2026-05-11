import { ref } from 'vue'
import { parseWasmError } from '@/types'
import init, { process_batch } from '@wasm/litemark_wasm.js'

const isReady = ref(false)
const error = ref<string | null>(null)
const isInitializing = ref(false)

async function initialize(): Promise<void> {
  if (isReady.value) return
  isInitializing.value = true
  error.value = null
  try {
    const wasmUrl = `${import.meta.env.BASE_URL}wasm/litemark_wasm_bg.wasm`
    await init({ module_or_path: wasmUrl })
    isReady.value = true
  } catch (err) {
    const wasmErr = parseWasmError(err)
    error.value = wasmErr.message
    throw wasmErr
  } finally {
    isInitializing.value = false
  }
}

export function useWasm() {
  return {
    isReady,
    error,
    isInitializing,
    initialize,
    process_batch,
  }
}
