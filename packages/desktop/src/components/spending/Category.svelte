<!--
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (C) 2025 Wakunguma Kalimukwa
-->
<script lang="ts">
    import { getBudget, totalSpent } from "$lib/api/category";
    import { categoryStore } from "$lib/stores/categories.svelte";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import type { Budget, Category } from "$lib/types";
    import { formatMoney } from "$lib/utils/money";

    // TODO: test this
    // TODO: add chevron for collapsing

    type Props = {
        budget: Budget;
        category: Category;
    };

    const { budget, category }: Props = $props();

    // TODO: add border when editing
    // - maybe add text box component or something like that
    // TODO: maybe add box shadow for transactions instead of popup?
    // - then revert to old value on failure

    // TODO: test if reactive
    // TODO: maybe extract into functions?
    const amount = parseFloat(budget.amount);
    const total = totalSpent(category.id, transactionStore.transactions);
    const leftToSpend = Math.max(parseFloat(budget.amount) - total, 0);
    const percentage = Math.min(
        Math.round((total / Math.max(parseFloat(budget.amount), 1)) * 100),
        100,
    );
</script>

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
