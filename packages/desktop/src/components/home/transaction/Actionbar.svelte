<script lang="ts">
    import { TextButton,IconButton } from "$components/button";
    import { Trash2, X } from "@lucide/svelte";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { TableStore } from "$lib/stores/table.svelte";
    import { fly } from "svelte/transition";

    interface Props{
        tableStore: TableStore
    }

    // TODO: fix divider
    const { tableStore }:Props = $props();
    const visible = $derived(tableStore.selectedRows.size > 0);

    async function deleteTransactions(){
        if (tableStore.allRowsSelected){
            const ids = transactionStore.transactions.map(t => t.id);
            await transactionStore.deleteTransactions(ids);
        }
        await transactionStore.deleteTransactions(Array.from(tableStore.selectedRows));
        tableStore.clear();
    }
</script>

{#if visible || tableStore.allRowsSelected}
    <div class="action-bar" transition:fly={{ y:200,duration: 250 }}>
        <!---TODO: test this -->
        {#if tableStore.selectedRows.size === 1}
            <p>1 transaction</p>
        {:else if tableStore.allRowsSelected}
            <p>{transactionStore.transactions.length} transactions</p>
        {:else}
            <p>{tableStore.selectedRows.size} transactions</p>
        {/if}
        <div class="w-px h-full bg-neutral-50"></div>
        <TextButton onclick={deleteTransactions}>
            <Trash2/>
            Delete
        </TextButton>
        <div class="w-px h-full bg-neutral-50"></div>
        <IconButton aria-label="Close" variant="ghost" onclick={() => tableStore.clear()}>
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
        left: 50%;
        transform: translateX(-50%);
    }
</style>
