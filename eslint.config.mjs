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
            "svelte/no-at-html-tags": "warn",
            "svelte/valid-compile": "warn",
            "svelte/no-navigation-without-resolve": "off",
        },
    },
    {
        rules: {
            "indent": ["warn", 4],
            "quotes": ["warn", "double"],
            "semi": ["error", "always"],
            "comma-dangle": ["warn", "always-multiline"],
            "object-curly-spacing": ["warn", "always"],
            "array-bracket-spacing": ["warn", "never"],
            "no-trailing-spaces": "warn",
            "no-multiple-empty-lines": ["warn", { max: 1 }],
        },
    },
];
