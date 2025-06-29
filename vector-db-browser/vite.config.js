import { defineConfig } from 'vite';
import path from 'path';

export default defineConfig({
  resolve: {
    alias: {
      'onnxruntime-node': path.resolve(__dirname, './empty-module.js')
    }
  },
  server: {
    port: 5500
  },
   optimizeDeps: {
    exclude: ['onnxruntime-node', '@xenova/transformers',]
  },
  build: {
    commonjsOptions: {
      exclude: ['onnxruntime-node', '@xenova/transformers'],
    },
    rollupOptions: {
      external: ['onnxruntime-node'], // Skip bundling it
    },
  },
  define: {
    global: {}
  }
});
