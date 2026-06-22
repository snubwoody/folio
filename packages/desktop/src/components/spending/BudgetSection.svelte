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
    import { getBudget, totalSpent } from "$lib/api/category";
    import { categoryStore } from "$lib/stores/categories.svelte";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import type { Category, Budget } from "$lib/types";
    import { formatMoney } from "$lib/utils/money";
    // import Budget from "./Budget.svelte";
    // TODO: add default category groups?
    // TODO: test this
    // TODO: add chevron for collapsing
    $inspect(categoryStore.categories);
    const categoriesWithoutGroup = $derived(
        categoryStore.categories.filter((c) => !c.categoryGroupId),
    );

    // TODO: add border when editing
    // - maybe add text box component or something like that
    // TODO: maybe add box shadow for transactions instead of popup?
    // - then revert to old value on failure

    // TODO: test if reactive
    // TODO: maybe extract into functions?
</script>

{#snippet budget(category: Category, budget: Budget)}
    {@const amount = parseFloat(budget.amount)}
    {@const total = totalSpent(category.id, transactionStore.transactions)}
    {@const leftToSpend = Math.max(parseFloat(budget.amount) - total, 0)}
    {@const percentage = Math.min(
        Math.round((total / Math.max(parseFloat(budget.amount), 1)) * 100),
        100,
    )}
    <li class="category">
        <div class="max-w-[450px]">
            <div class="flex justify-between">
                <p>{category.title}</p>
                {#if total === amount && amount > 0}
                    <p>Fully spent</p>
                {:else if total > amount}
                    {@const excess = total - amount}
                    Overspent by {formatMoney(excess.toString())}
                {:else}
                    Spent {formatMoney(total.toString())} of {formatMoney(
                        budget.amount,
                    )}
                {/if}
            </div>
            <div class="budget-bar">
                <div
                    style={`--percentage:${percentage}%`}
                    class="budget-bar-thumb"
                ></div>
            </div>
        </div>
        <p>{formatMoney(budget.amount)}</p>
        <p>{formatMoney(leftToSpend.toString())}</p>
    </li>
{/snippet}

<section>
    <div class="category-group-header border-b border-b-neutral-100">
        <p class="font-medium">Categories</p>
        <p class="font-medium">Budgeted</p>
        <p class="font-medium">Remaining</p>
    </div>
    {#each categoryStore.categoryGroups as group (group.id)}
        {@const categories = categoryStore.categories.filter(
            (c) => c.categoryGroupId === group.id,
        )}
        <ul class="category-group">
            <div class="category-group-header">
                <p>{group.title}</p>
                <p>$0.00</p>
                <p>$0.00</p>
            </div>
            <!--TODO: use composite key -->
            {#each categories as category (category.id)}
                {#await getBudget(category.id)}
                    <div class="hidden"></div>
                {:then b}
                    {@render budget(category, b)}
                {/await}
            {/each}
        </ul>
    {/each}
    <ul class="category-group">
        <div class="category-group-header">
            <p>No group</p>
            <p>$0.00</p>
            <p>$0.00</p>
        </div>
        <!--TODO: use composite key -->
        {#each categoriesWithoutGroup as category (category.id)}
            {#await getBudget(category.id)}
                <div class="hidden"></div>
            {:then b}
                {@render budget(category, b)}
            {/await}
        {/each}
    </ul>
</section>

<style>
    section {
        display: grid;
        grid-template-columns: 1fr auto auto;
        column-gap: 12px;
    }

    .category-group-header {
        padding: 8px 16px;
        background: var(--color-neutral-25);
        display: grid;
        grid-column: 1 / -1;
        grid-template-columns: subgrid;
    }

    .category-group {
        display: grid;
        grid-column: 1 / -1;
        grid-template-columns: subgrid;
    }

    .category {
        display: grid;
        grid-column: 1 / -1;
        grid-template-columns: subgrid;
        padding-inline: 16px;
        padding-block: 8px;
    }

    .budget-table {
        display: grid;
        grid-template-columns: 1fr auto auto;
        align-items: center;
        gap: 16px 56px;
    }

    .budget-bar {
        background: var(--color-neutral-50);
        border-radius: var(--radius-full);
        width: 100%;
        height: 3px;
    }

    .budget-bar-thumb {
        background: var(--color-green-700);
        border-radius: var(--radius-full);
        width: var(--percentage);
        height: 100%;
    }
</style>
