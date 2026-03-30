import { defineConfig } from "oxlint";

export default defineConfig({
    
    categories: {
        correctness: "warn"
    },
    rules: {
        "typescript/no-unused-vars": "warn",
        "typescript/no-import-type-side-effects": "warn"
    }
});