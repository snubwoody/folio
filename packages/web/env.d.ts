/// <reference types="astro/client" />

interface ImportMetaEnv {
    readonly PERSONAL_ACCESS_TOKEN: string;
}

interface ImportMeta {
    readonly env: ImportMetaEnv;
}