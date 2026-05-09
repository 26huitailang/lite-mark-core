import { ref } from 'vue'

const heicSupported = ref<boolean | null>(null)

export function useHeic() {
  async function checkSupport(): Promise<boolean> {
    if (heicSupported.value !== null) return heicSupported.value
    const isSafari = /^((?!chrome|android).)*safari/i.test(navigator.userAgent)
    heicSupported.value = isSafari
    return isSafari
  }

  function isHeicFile(file: File): boolean {
    const name = file.name.toLowerCase()
    return name.endsWith('.heic') || name.endsWith('.heif') ||
      file.type === 'image/heic' || file.type === 'image/heif'
  }

  async function convertHeicToJpeg(file: File): Promise<File | null> {
    if (!isHeicFile(file)) return file

    try {
      const bitmap = await createImageBitmap(file)
      const canvas = document.createElement('canvas')
      canvas.width = bitmap.width
      canvas.height = bitmap.height
      const ctx = canvas.getContext('2d')!
      ctx.drawImage(bitmap, 0, 0)

      const blob = await new Promise<Blob | null>((resolve) =>
        canvas.toBlob(resolve, 'image/jpeg', 0.95)
      )
      if (!blob) throw new Error('Canvas toBlob failed')

      const newName = file.name.replace(/\.(heic|heif)$/i, '.jpg')
      return new File([blob], newName, { type: 'image/jpeg', lastModified: file.lastModified })
    } catch {
      try {
        const arrayBuffer = await file.arrayBuffer()
        const uint8 = new Uint8Array(arrayBuffer)
        if (uint8[0] === 0xFF && uint8[1] === 0xD8 && uint8[2] === 0xFF) {
          const newName = file.name.replace(/\.(heic|heif)$/i, '.jpg')
          return new File([arrayBuffer], newName, { type: 'image/jpeg', lastModified: file.lastModified })
        }
        return null
      } catch {
        return null
      }
    }
  }

  return { heicSupported, checkSupport, isHeicFile, convertHeicToJpeg }
}
