<template>
  <div class="result-card">
    <div class="result-image" @click="showCompare = true">
      <img :src="result.outputUrl" :alt="result.originalName" loading="lazy" />
      <div class="image-overlay">
        <button class="compare-btn" title="前后对比">
          🔍
        </button>
      </div>
    </div>

    <div class="result-info">
      <div class="result-name" :title="result.originalName">
        {{ result.originalName }}
      </div>
      <div class="result-meta">
        {{ formatFileSize(result.outputSize) }}
        <span v-if="result.elapsedMs > 0">
          · {{ result.elapsedMs }}ms
        </span>
      </div>
      <a
        class="download-link"
        :href="result.outputUrl"
        :download="`watermarked_${result.originalName.replace(/\.[^.]+$/, '')}.jpg`"
        @click.stop
      >
        📥 下载
      </a>
    </div>

    <ImageCompare
      v-if="showCompare && result.originalUrl"
      :original-url="result.originalUrl"
      :processed-url="result.outputUrl"
      @close="showCompare = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { ProcessResult } from '@/types'
import { formatFileSize } from '@/utils/fileValidation'
import ImageCompare from './ImageCompare.vue'

defineProps<{
  result: ProcessResult
}>()

const showCompare = ref(false)
</script>

<style scoped>
.result-card {
  border: 1px solid #e2e8f0;
  border-radius: 10px;
  overflow: hidden;
  background: white;
  transition: box-shadow 0.2s;
}

.result-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}

.result-image {
  position: relative;
  aspect-ratio: 4 / 3;
  overflow: hidden;
  cursor: pointer;
}

.result-image img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

.image-overlay {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.3);
  opacity: 0;
  transition: opacity 0.2s;
}

.result-image:hover .image-overlay {
  opacity: 1;
}

.compare-btn {
  width: 44px;
  height: 44px;
  border: none;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.9);
  font-size: 20px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

.result-info {
  padding: 10px 12px;
}

.result-name {
  font-size: 12px;
  color: #333;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.result-meta {
  font-size: 11px;
  color: #999;
  margin-top: 2px;
}

.download-link {
  display: inline-block;
  margin-top: 6px;
  font-size: 12px;
  color: #007AFF;
  text-decoration: none;
  font-weight: 500;
}

.download-link:hover {
  text-decoration: underline;
}
</style>
