// @ts-check
import { defineConfig } from "astro/config";
import tailwindcss from "@tailwindcss/vite";
import sitemap from "@astrojs/sitemap";
import node from "@astrojs/node";

export default defineConfig({
    prefetch: true,
    integrations: [sitemap()],
    vite: {
        // what a horrible language
        // @ts-expect-error for some reason ts can't see this
        plugins: [tailwindcss()]
    },
    adapter: node({
        mode: "standalone"
    })
});