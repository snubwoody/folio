<script lang="ts">
    import { Popover } from "bits-ui";
    import type { Snippet } from "svelte";

    interface Props{
        children: Snippet,
        class?: string,
    }

    const { children,class:userClass }: Props = $props();
</script>

<Popover.Portal>
    <Popover.Content class={`popup-overlay ${userClass??""}`}>
        {@render children()}
    </Popover.Content>
</Popover.Portal>
