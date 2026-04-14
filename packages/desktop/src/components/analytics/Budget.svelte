<!--
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (C) 2025 Wakunguma Kalimukwa
-->
<script lang="ts">
    import MoneyCell from "$components/MoneyCell.svelte";
    import type { Budget } from "$lib/types";
    import { appStore } from "$lib/state.svelte";
    import { settingsStore } from "$lib/stores/settings.svelte";
    import { formatAmountWithoutSymbol, formatMoney, getCurrencySymbol } from "$lib/utils/money";
    import {totalSpent} from "$lib/api/category";
    import {transactionStore} from "$lib/stores/transaction.svelte";
    import {getLocalTimeZone, today} from "@internationalized/date";

    type Props = {
        budget: Budget;
    };

    const { budget }: Props = $props();
    // TODO: store as number
    // TODO: create a budget for every category
    // FIXME: overspent error
    // FIXME: add a budget_amount field to categories
    const total = $derived(totalSpent(budget.category.id,transactionStore.transactions,today(getLocalTimeZone())));
    let amount = $derived(parseFloat(budget.amount));
    let leftToSpend = $derived(Math.max(amount-total,0));

    /// Max 1 to prevent NaN and mess up the bar width
    let percentage = $derived(Math.min(Math.round((total / Math.max(amount,1)) * 100),100));

    const formattedAmount = $derived.by(() =>
        formatAmountWithoutSymbol(budget.amount)
    );

    async function updateAmount(newAmount: string) {
        await appStore.editBudget(budget.id, newAmount);
        amount = parseFloat(newAmount);
    }
</script>

<div class="flex flex-col relative gap-1.5 max-w-[600px]">
    <div class="flex items-center justify-between">
        <p>{budget.category?.title ?? " "}</p>
        {#if total === amount && amount > 0}
            <p>Fully spent</p>
        {:else if total > amount}
            {@const excess = total -amount}
            Overspent by {formatMoney(excess.toString())}
        {:else}
            Spent {formatMoney(budget.totalSpent)} of {formatMoney(budget.amount)}
        {/if}
    </div>
    <div>
        <div class="budget-bar">
            <div style={`--percentage:${percentage}%`} class="budget-bar-thumb"></div>
        </div>
    </div>
</div>
<MoneyCell symbol={getCurrencySymbol(settingsStore.settings.currencyCode)} amount={formattedAmount} onUpdate={updateAmount} />
<p>
    {formatMoney(leftToSpend.toString())}
</p>

<style>
    .budget-bar{
        background: var(--color-neutral-50);
        border-radius: var(--radius-full);
        width: 100%;
        height: 8px;
    }

    .budget-bar-thumb{
        background: var(--color-green-700);
        border-radius: var(--radius-full);
        width: var(--percentage);
        height: 100%;
    }
</style>
