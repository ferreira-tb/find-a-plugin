import tailwind from 'tailwindcss';
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import autoprefixer from 'autoprefixer';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
  clearScreen: false,
  plugins: [wasm(), sveltekit()],
  css: {
    postcss: {
      plugins: [tailwind(), autoprefixer()],
    },
  },
  build: {
    emptyOutDir: true,
    minify: true,
    target: 'esnext',
  },
});
