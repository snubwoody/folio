// @ts-check
import { defineConfig } from "astro/config";
import tailwindcss from "@tailwindcss/vite";
import sitemap from "@astrojs/sitemap";
import node from "@astrojs/node";

// https://astro.build/config
export default defineConfig({
    prefetch: true,
    vite: {
        plugins: [tailwindcss(),sitemap()]
    },
    adapter: node({
        mode: "standalone"
    })
});