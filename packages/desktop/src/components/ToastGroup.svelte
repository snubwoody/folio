<script lang="ts">
    import { toastStore } from "$lib/toast.svelte";
    // TODO: close toast on esc
</script>

<div class="toast-group">
    {#each toastStore.toasts as toast}
        <div class="toast">
            {toast.title}
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
    }
</style>