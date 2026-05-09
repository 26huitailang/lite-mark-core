import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { ValidatedFile, RejectedFile } from '@/types'
import { useHeic } from '@/composables/useHeic'

export const useFilesStore = defineStore('files', () => {
  const selectedFiles = ref<ValidatedFile[]>([])
  const logoFile = ref<File | null>(null)
  const rejectedFiles = ref<RejectedFile[]>([])

  const hasFiles = computed(() => selectedFiles.value.length > 0)
  const fileCount = computed(() => selectedFiles.value.length)

  const { checkSupport, isHeicFile, convertHeicToJpeg } = useHeic()

  async function addFiles(files: File[]): Promise<void> {
    rejectedFiles.value = []
    const supported = await checkSupport()
    const validated: ValidatedFile[] = []

    for (const file of files) {
      const isHeic = isHeicFile(file)
      if (isHeic && !supported) {
        rejectedFiles.value.push({ name: file.name, reason: 'heic-unsupported' })
        continue
      }

      let finalFile = file
      if (isHeic) {
        const converted = await convertHeicToJpeg(file)
        if (converted) {
          finalFile = converted
        } else {
          rejectedFiles.value.push({ name: file.name, reason: 'conversion-failed' })
          continue
        }
      }

      validated.push({
        file: finalFile,
        originalName: file.name,
        isHeic,
        size: finalFile.size,
      })
    }

    selectedFiles.value.push(...validated)
  }

  function removeFile(index: number): void {
    selectedFiles.value.splice(index, 1)
  }

  function clearFiles(): void {
    selectedFiles.value = []
    rejectedFiles.value = []
  }

  function setLogo(file: File | null): void {
    logoFile.value = file
  }

  return {
    selectedFiles,
    logoFile,
    rejectedFiles,
    hasFiles,
    fileCount,
    addFiles,
    removeFile,
    clearFiles,
    setLogo,
  }
})
