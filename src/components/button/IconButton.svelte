<script lang="ts">
    import { type Snippet } from "svelte";
    import "$styles/icon-btn.css";

    type Style = "primary" | "ghost" | "neutral";
    type Size = "small" | "medium";

    type Props = {
        children: Snippet
        variant?: Style
        size?: Size,
        href?: string,
        class?: string
    };

    const {
        children,
        size = "medium",
        variant = "primary",
        href,
        class:userClass,
    }: Props = $props();

</script>

{#if href}
    <a {href} class={`icon-btn icon-btn-${variant} icon-btn-${size} ${userClass}`}>
        {@render children()}
    </a>
{:else}
    <button class={`icon-btn icon-btn-${variant} icon-btn-${size} ${userClass}`}>
        {@render children()}
    </button>
{/if}

