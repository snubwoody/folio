// @ts-check
import { defineConfig } from 'astro/config';
import tailwindcss from '@tailwindcss/vite';
import sitemap from "@astrojs/sitemap";
import vercel from "@astrojs/vercel";
import node from "@astrojs/node";

// https://astro.build/config
export default defineConfig({
    prefetch: true,
    vite: {
      plugins: [tailwindcss(),sitemap()],
    },
    adapter: !(process.env.VERCEL_ENV === "production" || process.env.VERCEL_ENV === "preview") ? node({
      mode: "standalone"
    }) : vercel()
});