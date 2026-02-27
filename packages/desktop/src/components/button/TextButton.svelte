<script lang="ts">
    import type { HTMLButtonAttributes } from "svelte/elements";
    import type { Snippet } from "svelte";

    type Theme = "primary" | "neutral";

    interface Props extends HTMLButtonAttributes{
        theme?: Theme,
        children: Snippet
    }

    const {theme = "neutral",children,class:userClass,...rest}:Props = $props()
</script>

<!--TODO: test data-theme-->
<button data-theme={theme} class={`text-button ${userClass}`} {...rest}>
    {@render children()}
</button>

