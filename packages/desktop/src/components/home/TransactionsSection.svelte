<!--
Copyright (C) 2025 Wakunguma Kalimukwa

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
-->
<script lang="ts">
    import { Tabs } from "melt/builders";
    import TransactionTable from "./expense/ExpenseTable.svelte";
    import AddTransaction from "./AddTransaction.svelte";
    import IncomeTable from "./income/IncomeTable.svelte";
    import { SegmentedTabs,TabContent,TabButton } from "$components/select";

    type TransactionTab = "Expenses" | "Income";
    const tabIds: TransactionTab[] = ["Expenses", "Income"];
    let activeTab = $state(tabIds[0]);
    const tabs = new Tabs<TransactionTab>({ value: tabIds[0],onValueChange: (active) => activeTab = active });
    let active = $state("Expenses");
</script>

<section class="space-y-2">
    <SegmentedTabs bind:value={active}>
        <TabButton value="Expenses">Expenses</TabButton>
        <TabButton value="Incomes">Incomes</TabButton>
        <TabContent value="Expenses">Expenses</TabContent>
        <TabContent value="Incomes">Incomes</TabContent>
    </SegmentedTabs>

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
