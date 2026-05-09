import JSZip from 'jszip'
import type { ProcessResult } from '@/types'

export async function createResultsZip(results: ProcessResult[]): Promise<Blob> {
  const zip = new JSZip()
  for (const r of results) {
    const baseName = r.originalName.replace(/\.[^.]+$/, '')
    const name = `watermarked_${baseName}.jpg`
    zip.file(name, r.outputBytes)
  }
  return zip.generateAsync({ type: 'blob' })
}

export function downloadBlob(blob: Blob, filename: string): void {
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = filename
  a.click()
  URL.revokeObjectURL(url)
}
