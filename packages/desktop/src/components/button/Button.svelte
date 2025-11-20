<script lang="ts">
    import type { HTMLButtonAttributes } from "svelte/elements";
    import type {Snippet} from "svelte";
    type Variant = "primary" | "secondary" | "neutral" | "bordered";

    interface Props extends HTMLButtonAttributes{
        variant?: Variant,
        children: Snippet
    }

    const {
        variant = "primary",
        children,
        class:userClass,
        ...rest
    }: Props = $props();
</script>

<button class={`btn btn-${variant} ${userClass}`} {...rest}>
    {@render children()}
</button>

