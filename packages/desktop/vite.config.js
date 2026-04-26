import { defineConfig } from "vitest/config";
import { sveltekit } from "@sveltejs/kit/vite";
import { playwright } from "@vitest/browser-playwright";
import tailwind from "@tailwindcss/vite";

// eslint-disable-next-line
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
    plugins: [
        sveltekit(),
        tailwind()
    ],
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
                port: 1421
            }
            : undefined,
        watch: {
            ignored: ["**/src-tauri/**"]
        }
    },
    test: {
        exclude: ["e2e", "node_modules", ".vercel", "dist"],
        // eslint-disable-next-line no-undef
        reporters: process.env.CI ? ["verbose","github-actions"] : "verbose",
        // *.test.ts are for unit tests, *.spec.ts are for browser tests
        projects: [
            {
                extends: true,
                test: {
                    name: "unit",
                    include: ["src/**/*.test.ts"],
                    environment: "jsdom"
                }
            },
            {
                extends: true,
                test: {
                    include: ["src/**/*.spec.ts"],
                    name: "browser",
                    environment: "jsdom",
                    browser: {
                        enabled: true,
                        provider: playwright(),
                        screenshotFailures: false,
                        api: {
                            host: "127.0.0.1",
                            port: 4040
                        },
                        instances: [
                            { browser: "chromium" },
                            { browser: "webkit" },
                        ]
                    }
                }
            }
        ]
    }
});
