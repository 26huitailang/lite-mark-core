import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'

export default defineConfig(({ mode }) => ({
  plugins: [vue()],
  base: mode === 'production' ? '/lite-mark-core/' : '/',
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@wasm': path.resolve(__dirname, '../pkg'),
    },
  },
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    assetsInlineLimit: 0,
  },
  server: {
    fs: {
      allow: ['..'],
    },
  },
}))