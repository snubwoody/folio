<script lang="ts">
    import { Info, X } from "@lucide/svelte";
    import { IconButton } from "$components/button";
    import type { Snippet } from "svelte";

    type Props = {
        message: string,
        children?:Snippet,
        open?: boolean
    }

    let {
        message,
        children,
        open = $bindable(false)
    }: Props = $props();
</script>

<div class="message-bar">
    <Info size="20" class="text-text-primary shrink-0"/>
    <p class="w-full">{message}</p>
    {@render children?.()}
    <IconButton variant="ghost" onclick={() => open = false}>
        <X/>
    </IconButton>
</div>

<style>
    .message-bar{
        display: flex;
        border: 1px solid var(--color-border-neutral-10);
        border-radius: var(--radius-md);
        padding: 12px;
        gap: 12px;
        align-items: center;
    }
</style>