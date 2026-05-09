declare module '@wasm/litemark_wasm.js' {
  export default function init(): Promise<void>
  export function process_image(
    imageBytes: Uint8Array,
    templateJson: string,
    author?: string,
    fontBytes?: Uint8Array,
    logoBytes?: Uint8Array
  ): Uint8Array
  export function process_image_basic(
    imageBytes: Uint8Array,
    templateJson: string
  ): Uint8Array
  export function process_batch(
    images: Uint8Array[],
    templateJson: string,
    author?: string,
    fontBytes?: Uint8Array,
    logoBytes?: Uint8Array,
    onProgress?: (completed: number, total: number) => void
  ): Uint8Array[]
}
