<script lang="ts">
    import {Popover} from "bits-ui";
    import type { Snippet } from "svelte";


    let {
        children,
        open = $bindable(),
        ...rest
    }: Popover.RootProps = $props();
</script>

<Popover.Root bind:open={open} {...rest}>
    {@render children?.()}
</Popover.Root>