<template>
  <div v-if="filesStore.hasFiles" class="file-list">
    <div class="file-list-header">
      <span class="file-count">已选择 {{ filesStore.fileCount }} 张图片</span>
      <button class="clear-btn" @click="filesStore.clearFiles()">清空</button>
    </div>
    <div class="file-items">
      <div v-for="(file, index) in filesStore.selectedFiles" :key="file.originalName + index" class="file-item">
        <div class="file-thumb">
          <img v-if="thumbUrls[index]" :src="thumbUrls[index]" alt="" />
          <span v-else class="file-icon">🖼️</span>
        </div>
        <div class="file-info">
          <div class="file-name" :title="file.originalName">{{ file.originalName }}</div>
          <div class="file-meta">
            {{ formatFileSize(file.size) }}
            <span v-if="file.isHeic" class="heic-badge">HEIC</span>
          </div>
        </div>
        <button class="remove-btn" @click="filesStore.removeFile(index)" title="移除">✕</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useFilesStore } from '@/stores/files'
import { formatFileSize } from '@/utils/fileValidation'

const filesStore = useFilesStore()

const thumbUrls = computed(() =>
  filesStore.selectedFiles.map(f => URL.createObjectURL(f.file))
)
</script>

<style scoped>
.file-list {
  margin-bottom: 16px;
}

.file-list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.file-count {
  font-size: 13px;
  color: #666;
}

.clear-btn {
  padding: 4px 12px;
  border: none;
  background: transparent;
  color: #999;
  font-size: 12px;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s;
}

.clear-btn:hover {
  background: #f5f5f5;
  color: #666;
}

.file-items {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: #f8fafc;
  border-radius: 8px;
  border: 1px solid #e2e8f0;
  max-width: 280px;
}

.file-thumb {
  width: 40px;
  height: 40px;
  border-radius: 4px;
  overflow: hidden;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #e2e8f0;
}

.file-thumb img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.file-icon {
  font-size: 18px;
}

.file-info {
  flex: 1;
  min-width: 0;
}

.file-name {
  font-size: 12px;
  color: #333;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-meta {
  font-size: 11px;
  color: #999;
  margin-top: 2px;
  display: flex;
  align-items: center;
  gap: 6px;
}

.heic-badge {
  padding: 1px 5px;
  background: #fff3e0;
  color: #e65100;
  font-size: 10px;
  border-radius: 3px;
}

.remove-btn {
  width: 22px;
  height: 22px;
  border: none;
  background: transparent;
  color: #ccc;
  cursor: pointer;
  font-size: 14px;
  line-height: 1;
  border-radius: 4px;
  flex-shrink: 0;
  transition: all 0.2s;
}

.remove-btn:hover {
  background: #fee2e2;
  color: #ef4444;
}
</style>
