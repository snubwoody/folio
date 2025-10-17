import { defineConfig } from 'vitest/config'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import path from 'path'

export default defineConfig({
    plugins: [svelte()],
    resolve: {
    alias: {
      '@/': path.resolve(__dirname, './src/'),
    },
  },
    test: {
        browser: {
        enabled: true,
        headless: true,
        provider: 'playwright',
        // https://vitest.dev/guide/browser/playwright
        instances: [
        { browser: 'chromium' },
        { browser: 'firefox' },
        { browser: 'webkit' },
        ],
        },
    },
})
