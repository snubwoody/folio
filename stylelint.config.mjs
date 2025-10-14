/** @type {import('stylelint').Config} */
export default {
    extends: ["stylelint-config-standard"],
    overrides: [
        {
            // For Svelte <style> blocks
            files: ["**/*.svelte"],
            customSyntax: "postcss-html",
        },
    ],
    rules: {
        "color-no-invalid-hex": true,
        "alpha-value-notation": "number",
        "import-notation": "string",
        "custom-property-pattern": null,
        // Ignore tailind @ rules
        "at-rule-no-unknown": [true,{ ignoreAtRules: ["utility","theme"] }],
        // Relax some rules for Svelte
        "selector-pseudo-class-no-unknown": [
            true,
            {
                ignorePseudoClasses: ["global"],
            },
        ],
    },
};
