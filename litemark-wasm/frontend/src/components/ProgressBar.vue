<template>
  <div v-if="processingStore.isProcessing || processingStore.hasResults" class="progress-section">
    <div v-if="processingStore.isProcessing" class="progress-bar-wrapper">
      <div class="progress-info">
        <span class="progress-text">处理中 {{ processingStore.currentIndex }} / {{ processingStore.totalCount }}</span>
        <span class="progress-percent">{{ processingStore.progress }}%</span>
      </div>
      <div class="progress-bar">
        <div class="progress-fill" :style="{ width: processingStore.progress + '%' }"></div>
      </div>
    </div>
    <div v-else-if="processingStore.hasResults" class="progress-done">
      ✅ 完成 {{ processingStore.results.length }} 张图片
      <span v-if="processingStore.processingTimeMs > 0" class="time-info">
        ({{ (processingStore.processingTimeMs / 1000).toFixed(2) }}s)
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useProcessingStore } from '@/stores/processing'

const processingStore = useProcessingStore()
</script>

<style scoped>
.progress-section {
  margin: 16px 0;
}

.progress-bar-wrapper {
  padding: 12px;
  background: #f8fafc;
  border-radius: 8px;
}

.progress-info {
  display: flex;
  justify-content: space-between;
  margin-bottom: 6px;
}

.progress-text {
  font-size: 13px;
  color: #666;
}

.progress-percent {
  font-size: 13px;
  font-weight: 600;
  color: #007AFF;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background: #e2e8f0;
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #007AFF, #00C7FF);
  border-radius: 4px;
  transition: width 0.3s ease;
}

.progress-done {
  padding: 10px 12px;
  background: #ecfdf5;
  color: #059669;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
}

.time-info {
  color: #999;
  font-weight: normal;
  margin-left: 4px;
}
</style>
