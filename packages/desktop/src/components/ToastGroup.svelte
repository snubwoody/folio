<script lang="ts">
    import { toastStore } from "$lib/toast.svelte";
    import {Info,X} from "@lucide/svelte";
    import { IconButton, Button } from "$components/button";
    // TODO: close toast on esc
</script>

<div class="toast-group">
    {#each toastStore.toasts as toast}
        <div class="toast">
            <div class="flex gap-1">
                <div class="py-0.5">
                    <Info size="20" class="text-purple-500"/>
                </div>
                <div>
                    <p>{toast.title}</p>
                    {#if toast.body}
                        <p class="text-sm text-text-muted">A new update is available.</p>
                    {/if}
                </div>
                <IconButton variant="ghost" class="ml-auto">
                    <X size="16"/>
                </IconButton>
            </div>
            {#if toast.secondaryAction || toast.primaryAction}
                <div class="flex justify-end items-center gap-1.5">
                    {#if toast.secondaryAction}
                        <Button size="small" variant="ghost" onclick={toast.secondaryAction.action}>
                            {toast.secondaryAction.text}
                        </Button>
                    {/if}
                    {#if toast.primaryAction}
                        <Button size="small"  onclick={toast.primaryAction.action}>
                            {toast.primaryAction.text}
                        </Button>
                    {/if}
                </div>
            {/if}
        </div>
    {/each}
</div>

<style>
    .toast-group{
        position: fixed;
        display: flex;
        padding: 20px 40px;
        flex-direction: column-reverse;
        gap: 16px;
        bottom: 0;
        right: 0;
    }

    .toast{
        width: 100vw;
        max-width: 350px;
        display: flex;
        flex-direction: column;
        gap: 16px;
        padding: 16px;
        box-shadow: var(--shadow-md);
        border-radius: var(--radius-md);
        border: 1px solid var(--color-border-neutral-10);
    }
</style>