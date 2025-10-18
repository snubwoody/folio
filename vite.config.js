import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import tailwind from "@tailwindcss/vite";

// eslint-disable-next-line
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
    plugins: [sveltekit(),tailwind()],
    // Prevent Vite from obscuring rust errors
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                protocol: "ws",
                host,
                port: 1421,
            }
            : undefined,
        watch: {
            ignored: ["**/src-tauri/**"],
        },
    },
    test: {
        exclude: ["e2e","node_modules",".vercel","dist"],
        projects: [
            {
                extends: true,
                test: {
                    name: "unit",
                    include: ["src/tests/unit/**/*.test.ts"],
                    environment: "node",
                },
            },
            {
                extends: true,
                test:{
                    include: ["src/tests/browser/**/*.test.ts"],
                    name: "browser",
                    browser: {
                        enabled: true,
                        provider: "playwright",
                        instances: [
                            { browser: "chromium" },
                            { browser: "firefox" },
                            { browser: "webkit" },
                        ],
                    },
                },
            },
        ],
    },
});
