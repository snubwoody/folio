// @ts-check
import { defineConfig } from "astro/config";
import tailwindcss from "@tailwindcss/vite";
import sitemap from "@astrojs/sitemap";
import node from "@astrojs/node";

// https://astro.build/config
export default defineConfig({
    prefetch: true,
    integrations: [sitemap()],
    vite: {
        plugins: [tailwindcss()]
    },
    adapter: node({
        mode: "standalone"
    })
});