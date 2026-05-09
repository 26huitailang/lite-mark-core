import { ref, watch } from 'vue'

export function useLocalStorage<T>(key: string, defaultValue: T) {
  const stored = localStorage.getItem(key)
  const data = ref<T>(stored ? JSON.parse(stored) : defaultValue)

  watch(
    data,
    (newVal) => {
      localStorage.setItem(key, JSON.stringify(newVal))
    },
    { deep: true }
  )

  return data
}
