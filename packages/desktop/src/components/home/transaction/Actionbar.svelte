<script lang="ts">
    import { TextButton,IconButton } from "$components/button";
    import { CirclePlus,Trash2,X } from "@lucide/svelte";
    import { transactionStore } from "$lib/transaction.svelte";
    import { today, getLocalTimeZone } from "@internationalized/date";
    import {TableStore} from "$lib/stores/table.svelte";

    interface Props{
        tableStore: TableStore
    }

    // TODO: add bulk delete
    // TODO: handle all selected
    const {tableStore}:Props = $props();
    const visible = $derived(tableStore.selectedRows.size > 0)
</script>

{#if visible}
    <div class="action-bar">
        <!---TODO: test this -->
        {#if tableStore.selectedRows.size === 1}
            <p>1 transaction</p>
        {:else}
            <p>{tableStore.selectedRows.size} transactions</p>
        {/if}
        <div class="w-[1px] h-full bg-neutral-50"></div>
        <TextButton>
            <Trash2/>
            Delete
        </TextButton>
        <div class="w-[1px] h-full bg-neutral-50"></div>
        <IconButton variant="ghost">
            <X />
        </IconButton>
    </div>
{/if}

<style>
    .action-bar{
        position: fixed;
        display: flex;
        gap: 16px;
        padding: 8px 16px;
        align-items: center;
        bottom: 24px;
        background: white;
        box-shadow: var(--shadow-purple-md);
        border-radius: var(--radius-md);
        border: 1px solid var(--color-purple-50);
    }
</style>
