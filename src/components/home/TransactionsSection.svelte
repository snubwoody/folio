<script lang="ts">
    import { Tabs } from "melt/builders";
    import TransactionTable from "./expense/ExpenseTable.svelte";
    import AddTransaction from "./AddTransaction.svelte";
    import IncomeTable from "./income/IncomeTable.svelte";
    
    const tabIds = ["Expenses", "Income",];
    const tabs = new Tabs({value: tabIds[0]})
</script>

<section class="space-y-2">
    <header class="flex items-center gap-2">
        {#each tabIds as id}
            <button {...tabs.getTrigger(id)}>
            {id}
            </button>
        {/each}
        <AddTransaction/>
    </header>
    {#each tabIds as id}
        <div {...tabs.getContent(id)}>
            {#if id === "Expenses"}
                <TransactionTable/>
            {:else}
                <IncomeTable/>
            {/if}
        </div>
    {/each}
</section>

<style>
    [data-melt-tabs-trigger]{
        padding: 8px 16px;
        transition: all 250ms;
        border-radius: 999px;
        color: var(--color-text-muted);

        &[data-active]{
            background-color: var(--color-neutral-50);
            color: var(--color-text-body);
        }
    }
</style>
