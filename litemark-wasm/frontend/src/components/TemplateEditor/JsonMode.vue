<template>
  <div class="json-mode">
    <div ref="editorRef" class="json-editor"></div>
    <p class="json-hint">提示：支持 {Author}, {Camera}, {Lens}, {Focal}, {Aperture}, {Shutter}, {ISO}, {DateTime} 变量</p>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { EditorView, basicSetup } from 'codemirror'
import { json } from '@codemirror/lang-json'
import { oneDark } from '@codemirror/theme-one-dark'
import { useTemplateStore } from '@/stores/template'

const templateStore = useTemplateStore()
const editorRef = ref<HTMLDivElement>()
let editor: EditorView | null = null

onMounted(() => {
  if (!editorRef.value) return

  editor = new EditorView({
    doc: templateStore.jsonText,
    extensions: [
      basicSetup,
      json(),
      oneDark,
      EditorView.theme({
        '&': { fontSize: '13px' },
        '.cm-scroller': { fontFamily: 'SF Mono, Monaco, monospace' },
      }),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          templateStore.updateJsonText(update.state.doc.toString())
        }
      }),
    ],
    parent: editorRef.value,
  })
})

onUnmounted(() => {
  editor?.destroy()
})

// Sync from external changes (e.g., preset switch)
watch(() => templateStore.jsonText, (newText) => {
  if (editor && editor.state.doc.toString() !== newText) {
    editor.dispatch({
      changes: {
        from: 0,
        to: editor.state.doc.length,
        insert: newText,
      },
    })
  }
})
</script>

<style scoped>
.json-mode {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.json-editor {
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  overflow: hidden;
  min-height: 300px;
}

.json-editor :deep(.cm-editor) {
  min-height: 300px;
}

.json-hint {
  font-size: 11px;
  color: #999;
  margin: 0;
}
</style>
