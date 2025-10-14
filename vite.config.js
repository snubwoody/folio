import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import tailwind from "@tailwindcss/vite";

// eslint-disable-next-line
const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
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
}));
