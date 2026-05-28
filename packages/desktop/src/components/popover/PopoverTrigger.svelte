<script lang="ts">
    import type { Popover } from "bits-ui";
    import type { Snippet } from "svelte";

    interface Props extends Popover.TriggerProps {
        children: Snippet;
    }

    const { children, ...rest }: Props = $props();
</script>

<Popover.Trigger {...rest}>
    {@render children()}
</Popover.Trigger>