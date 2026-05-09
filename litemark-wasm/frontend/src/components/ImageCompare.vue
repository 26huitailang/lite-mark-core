<template>
  <Teleport to="body">
    <div class="compare-modal" @click="emit('close')">
      <div class="compare-content" @click.stop>
        <div class="compare-header">
          <span>前后对比</span>
          <button class="close-btn" @click="emit('close')">✕</button>
        </div>

        <div
          ref="containerRef"
          class="compare-container"
          @mousemove="onMove"
          @touchmove.prevent="onTouchMove"
        >
          <img :src="processedUrl" class="compare-image processed" alt="处理后" />
          <div class="clip-layer" :style="{ clipPath: `inset(0 ${100 - clipPercent}% 0 0)` }">
            <img :src="originalUrl" class="compare-image original" alt="原图" />
          </div>
          <div class="slider-handle" :style="{ left: clipPercent + '%' }">
            <div class="slider-line" />
            <div class="slider-knob">◀ ▶</div>
          </div>
        </div>

        <div class="compare-labels">
          <span>原图</span>
          <span>处理后</span>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

defineProps<{
  originalUrl: string
  processedUrl: string
}>()

const emit = defineEmits<{
  close: []
}>()

const containerRef = ref<HTMLDivElement>()
const clipPercent = ref(50)

function updateClip(clientX: number) {
  if (!containerRef.value) return
  const rect = containerRef.value.getBoundingClientRect()
  const x = clientX - rect.left
  clipPercent.value = Math.max(0, Math.min(100, (x / rect.width) * 100))
}

function onMove(e: MouseEvent) {
  updateClip(e.clientX)
}

function onTouchMove(e: TouchEvent) {
  updateClip(e.touches[0].clientX)
}

onMounted(() => {
  // Prevent body scroll
  document.body.style.overflow = 'hidden'
})

onUnmounted(() => {
  document.body.style.overflow = ''
})
</script>


<style scoped>
.compare-modal {
  position: fixed;
  inset: 0;
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.8);
  padding: 20px;
}

.compare-content {
  max-width: 90vw;
  max-height: 90vh;
  background: #1a1a1a;
  border-radius: 12px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.compare-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  color: white;
  font-size: 14px;
  font-weight: 500;
}

.close-btn {
  width: 28px;
  height: 28px;
  border: none;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
}

.close-btn:hover {
  background: rgba(255, 255, 255, 0.2);
}

.compare-container {
  position: relative;
  flex: 1;
  overflow: hidden;
  cursor: ew-resize;
}

.compare-image {
  width: 100%;
  height: auto;
  max-height: 70vh;
  object-fit: contain;
  display: block;
}

.compare-image.processed {
  position: relative;
}

.clip-layer {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.clip-layer .compare-image {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.slider-handle {
  position: absolute;
  top: 0;
  bottom: 0;
  width: 0;
  transform: translateX(-50%);
  pointer-events: none;
}

.slider-line {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 50%;
  width: 2px;
  background: white;
  transform: translateX(-50%);
}

.slider-knob {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: white;
  color: #333;
  font-size: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  letter-spacing: 2px;
}

.compare-labels {
  display: flex;
  justify-content: space-between;
  padding: 8px 16px;
  color: #999;
  font-size: 12px;
}
</style>
