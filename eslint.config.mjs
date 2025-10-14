import js from "@eslint/js";
import tseslint from "typescript-eslint";
import sveltePlugin from "eslint-plugin-svelte";
import svelteParser from "svelte-eslint-parser";

/** @type {import("eslint").Linter.Config[]} */
export default [
    // Base JS recommended rules
    js.configs.recommended,

    // TypeScript configs
    ...tseslint.configs.recommended,

    // Svelte configs
    ...sveltePlugin.configs["flat/recommended"],

    // Global ignores
    {
        ignores: [
            "**/node_modules",
            "**/dist/**",
            "**/build/**",
            ".svelte-kit",
            "**/target",
        ],
    },

    // TypeScript files
    {
        files: ["**/*.ts", "**/*.tsx"],
        languageOptions: {
            parser: tseslint.parser,
            parserOptions: {
                project: "./tsconfig.json",
            },
        },
        rules: {
            // Add any custom TS rules here
            "@typescript-eslint/no-unused-vars": ["warn"],
        },
    },

    // Svelte files
    {
        files: ["**/*.svelte"],
        languageOptions: {
            parser: svelteParser,
            parserOptions: {
                parser: tseslint.parser,
            },
        },
        rules: {
            // Svelte-specific rules
            "svelte/no-at-html-tags": "warn",
            "svelte/valid-compile": "error",
        },
    },
    {
        rules: {
            "indent": ["warn", 4],
            "quotes": ["warn", "double"],
            "semi": ["error", "always"],
            "comma-dangle": ["warn", "always-multiline"],
            "object-curly-spacing": ["error", "always"],
            "array-bracket-spacing": ["error", "never"],
            "no-trailing-spaces": "error",
            "eol-last": ["error", "always"],
            "no-multiple-empty-lines": ["error", { max: 1 }],
        },
    },
];
