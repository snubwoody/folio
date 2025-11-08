// @ts-check
import { defineConfig } from "astro/config";
import tailwindcss from "@tailwindcss/vite";
import sitemap from "@astrojs/sitemap";

export default defineConfig({
    prefetch: true,
    site:"https://snubwoody.github.io/folio/",
    base: "/folio",
    output: "static",
    integrations: [sitemap()],
    vite: {
        // @ts-expect-error for some reason ts can't see this
        plugins: [tailwindcss()]
    }
});