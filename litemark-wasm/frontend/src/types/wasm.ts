export interface ProcessResult {
  id: string
  originalName: string
  outputBytes: Uint8Array
  outputUrl: string
  originalUrl: string | null
  outputSize: number
  elapsedMs: number
}

export interface ValidatedFile {
  file: File
  originalName: string
  isHeic: boolean
  size: number
}

export interface RejectedFile {
  name: string
  reason: 'heic-unsupported' | 'conversion-failed' | 'invalid-type'
}

export type WasmErrorPhase = 'init' | 'decode' | 'exif' | 'template' | 'render' | 'encode'

export class WasmError extends Error {
  constructor(
    public phase: WasmErrorPhase,
    public original: unknown
  ) {
    const msg = original instanceof Error ? original.message : String(original)
    super(`WASM ${phase} error: ${msg}`)
  }
}

export function parseWasmError(err: unknown): WasmError {
  const msg = err instanceof Error ? err.message : String(err)
  const phase: WasmErrorPhase =
    msg.includes('decode') ? 'decode'
    : msg.includes('exif') || msg.includes('EXIF') ? 'exif'
    : msg.includes('template') ? 'template'
    : msg.includes('render') ? 'render'
    : msg.includes('encode') ? 'encode'
    : 'init'
  return new WasmError(phase, err)
}
