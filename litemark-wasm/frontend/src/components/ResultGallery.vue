<template>
  <div v-if="processingStore.hasResults" class="result-gallery">
    <div class="gallery-header">
      <span class="gallery-title">
        🎨 处理结果 ({{ processingStore.results.length }})
      </span>
      <button
        v-if="processingStore.results.length >= 2"
        class="zip-btn"
        @click="downloadZip"
      >
        📦 打包下载
      </button>
    </div>

    <div class="gallery-grid">
      <ResultCard
        v-for="result in processingStore.results"
        :key="result.id"
        :result="result"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useProcessingStore } from '@/stores/processing'
import { createResultsZip, downloadBlob } from '@/utils/zipDownload'
import ResultCard from './ResultCard.vue'

const processingStore = useProcessingStore()

async function downloadZip() {
  const blob = await createResultsZip(processingStore.results)
  downloadBlob(blob, `litemark_batch_${Date.now()}.zip`)
}
</script>

<style scoped>
.result-gallery {
  margin-top: 20px;
}

.gallery-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.gallery-title {
  font-size: 15px;
  font-weight: 600;
  color: #333;
}

.zip-btn {
  padding: 6px 14px;
  border: 1px solid #007AFF;
  background: #eff6ff;
  color: #007AFF;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.zip-btn:hover {
  background: #007AFF;
  color: white;
}

.gallery-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  gap: 16px;
}
</style>
