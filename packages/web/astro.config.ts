// @ts-check
import { defineConfig } from "astro/config";
import tailwindcss from "@tailwindcss/vite";
import sitemap from "@astrojs/sitemap";

export default defineConfig({
    prefetch: true,
    output: "static",
    integrations: [sitemap()],
    vite: {
        // @ts-expect-error for some reason ts can't see this
        plugins: [tailwindcss()]
    }
});