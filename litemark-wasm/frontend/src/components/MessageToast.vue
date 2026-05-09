<template>
  <Teleport to="body">
    <div class="toast-container">
      <TransitionGroup name="toast">
        <div
          v-for="msg in messages"
          :key="msg.id"
          class="toast"
          :class="msg.type"
          @click="remove(msg.id)"
        >
          <span class="toast-icon">{{ iconFor(msg.type) }}</span>
          <span class="toast-text">{{ msg.text }}</span>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface ToastMessage {
  id: number
  text: string
  type: 'success' | 'error' | 'warning' | 'info'
}

const messages = ref<ToastMessage[]>([])
let nextId = 0

function iconFor(type: string): string {
  switch (type) {
    case 'success': return '✅'
    case 'error': return '❌'
    case 'warning': return '⚠️'
    default: return 'ℹ️'
  }
}

function show(text: string, type: ToastMessage['type'] = 'info', duration = 5000): void {
  const id = nextId++
  messages.value.push({ id, text, type })
  setTimeout(() => remove(id), duration)
}

function remove(id: number): void {
  const idx = messages.value.findIndex(m => m.id === id)
  if (idx >= 0) messages.value.splice(idx, 1)
}

function success(text: string, duration?: number) { show(text, 'success', duration) }
function error(text: string, duration?: number) { show(text, 'error', duration) }
function warning(text: string, duration?: number) { show(text, 'warning', duration) }
function info(text: string, duration?: number) { show(text, 'info', duration) }

defineExpose({ show, success, error, warning, info })
</script>

<style scoped>
.toast-container {
  position: fixed;
  top: 16px;
  right: 16px;
  z-index: 9999;
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-width: 360px;
}

.toast {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 16px;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  cursor: pointer;
  font-size: 14px;
  line-height: 1.5;
  backdrop-filter: blur(8px);
}

.toast.success { background: rgba(236, 253, 245, 0.95); border-left: 4px solid #059669; color: #059669; }
.toast.error { background: rgba(254, 242, 242, 0.95); border-left: 4px solid #ef4444; color: #ef4444; }
.toast.warning { background: rgba(255, 251, 235, 0.95); border-left: 4px solid #f59e0b; color: #b45309; }
.toast.info { background: rgba(239, 246, 255, 0.95); border-left: 4px solid #3b82f6; color: #1e40af; }

.toast-icon {
  font-size: 16px;
  flex-shrink: 0;
}

.toast-text {
  word-break: break-word;
}

.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(100%);
}
</style>
