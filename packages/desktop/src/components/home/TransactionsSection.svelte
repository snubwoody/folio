<script lang="ts">
    import { Tabs } from "melt/builders";
    import TransactionTable from "./expense/ExpenseTable.svelte";
    import AddTransaction from "./AddTransaction.svelte";
    import IncomeTable from "./income/IncomeTable.svelte";

    type TransactionTab = "Expenses" | "Income";
    const tabIds: TransactionTab[] = ["Expenses", "Income"];
    let activeTab = $state(tabIds[0]);
    const tabs = new Tabs<TransactionTab>({ value: tabIds[0],onValueChange: (active) => activeTab = active });
</script>

<section class="space-y-2">
    <header class="flex items-center gap-2">
        {#each tabIds as id,i (i)}
            <button {...tabs.getTrigger(id)}>
            {id}
            </button>
        {/each}
        <AddTransaction {activeTab}/>
    </header>
    {#each tabIds as id,i (i)}
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
