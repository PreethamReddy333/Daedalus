import { defineConfig } from 'vite'

export default defineConfig({
    base: './',  // Use relative paths for static_assets serving
    build: {
        outDir: 'dist',
        assetsDir: 'assets',
        rollupOptions: {
            input: 'index.html'
        }
    },
    server: {
        port: 3000
    }
})
