<!--
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (C) 2025 Wakunguma Kalimukwa
-->
<script lang="ts">
    import { formatAmountWithoutSymbol, getCurrencySymbol } from "$lib/lib";
    import MoneyCell from "$components/MoneyCell.svelte";
    import { formatAmount, type Budget } from "$lib/lib";
    import { appStore } from "$lib/state.svelte";

    type Props = {
        budget: Budget;
    };

    const { budget }: Props = $props();
    // TODO: store as number
    // TODO: create a budget for every category
    // FIXME: overspent error
    const totalSpent = parseFloat(budget.totalSpent);
    const amount = parseFloat(budget.amount);

    /// Max 1 to prevent NaN and mess up the bar width
    let percentage = Math.min(Math.round((totalSpent / Math.max(amount,1)) * 100),100);

    const formattedAmount = $derived.by(() =>
        formatAmountWithoutSymbol(budget.amount)
    );

    async function updateAmount(amount: string) {
        await appStore.editBudget(budget.id, amount);
    }
</script>

<div class="flex flex-col relative gap-1.5 max-w-[600px]">
    <div class="flex items-center justify-between">
        <p>{budget.category?.title ?? " "}</p>
        {#if totalSpent === amount && amount > 0}
            <p>Fully spent</p>
        {:else if totalSpent > amount}
            {@const excess = totalSpent -amount}
            Overspent by {formatAmount(excess.toString())}
        {:else}
            Spent {formatAmount(budget.totalSpent)} of {formatAmount(budget.amount)}
        {/if}
    </div>
    <div>
        <div class="budget-bar">
            <div style={`--percentage:${percentage}%`} class="budget-bar-thumb"></div>
        </div>
    </div>
</div>
<MoneyCell symbol={getCurrencySymbol(appStore.settings.currencyCode)} amount={formattedAmount} onUpdate={updateAmount} />
<p>
    {formatAmount(budget.remaining,{ currency: appStore.settings.currencyCode })}
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
