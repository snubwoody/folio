import js from "@eslint/js";
import tseslint from "typescript-eslint";
import sveltePlugin from "eslint-plugin-svelte";
import svelteParser from "svelte-eslint-parser";
import astro from "eslint-plugin-astro";
import globals from "globals";
import stylistic from "@stylistic/eslint-plugin";
import { defineConfig, globalIgnores } from "eslint/config";

export default defineConfig([
    globalIgnores([
        "**/node_modules",
        "**/dist",
        "**/build",
        "**/.svelte-kit",
        "**/.astro",
        "**/.vscode",
        "**/.github",
        "**/.idea",
        "**/target"
    ]),
    js.configs.recommended,
    tseslint.configs.recommended,
    astro.configs.recommended,
    sveltePlugin.configs.recommended,
    {
        files: ["**/*.ts", "**/*.tsx"],
        languageOptions: {
            parser: tseslint.parser,
            parserOptions: {
                project: "./tsconfig.json"
            }
        },
        rules: {
            "@typescript-eslint/no-unused-vars": ["warn"]
        }
    },
    {
        files: ["**/*.js"],
        languageOptions: {
            globals:{
                ...globals.browser
            }
        }
    },
    {
        files: ["**/*.svelte"],
        languageOptions: {
            parser: svelteParser,
            parserOptions: {
                projectService: true,
                extraFileExtensions: [".svelte"],
                parser: tseslint.parser
            }
        },
        rules: {
            "svelte/no-at-html-tags": "warn",
            "svelte/valid-compile": "warn",
            "svelte/no-navigation-without-resolve": "off"
        }
    },
    {
        files: ["**/*.{js,mjs,cjs,ts,astro,svelte}"],
        plugins: {
            js,
            "@stylistic":stylistic
        },
        rules: {
            "no-unused-vars": "off",
            "prefer-const": "off",
            "@typescript-eslint/no-unused-vars": ["warn"],
            "object-curly-spacing": ["warn", "always"],
            "array-bracket-spacing": ["warn", "never"],
            "no-trailing-spaces": "warn",
            "no-empty": "warn",
            "no-multiple-empty-lines": ["warn", { max: 1 }],
            "@stylistic/indent": ["warn", 4],
            "@stylistic/quotes": ["warn", "double"],
            "@stylistic/semi": ["error"],
            "@stylistic/arrow-spacing": ["warn"],
            "@stylistic/brace-style": ["error"],
            "@stylistic/comma-dangle": ["error","never"]
        }
    }
]);
