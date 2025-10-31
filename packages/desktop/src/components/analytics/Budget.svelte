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
    import { formatAmountWithoutSymbol, getCurrencySymbol } from "$lib/lib";
    import MoneyCell from "$components/MoneyCell.svelte";
    import IconButton from "$components/button/IconButton.svelte";
    import { formatAmount, type Budget } from "$lib/lib";
    import { appStore } from "$lib/state.svelte";
    import { Trash2 } from "@lucide/svelte";

    type Props = {
        budget: Budget;
    };

    const { budget }: Props = $props();
    let formattedAmount = $derived.by(() =>
        formatAmountWithoutSymbol(budget.amount),
    );

    async function updateAmount(amount: string) {
        await appStore.editBudget(budget.id, amount);
    }
</script>

<div class="data-cell flex justify-between items-center relative">
    <p>{budget.category?.title ?? " "}</p>
    <IconButton
        class="delete-btn"
        size="small"
        variant="ghost"
        onclick={() => appStore.deleteBudget(budget.id)}
    >
        <Trash2 />
    </IconButton>
</div>
<MoneyCell symbol={getCurrencySymbol(appStore.settings.currencyCode)} amount={formattedAmount} onUpdate={updateAmount} />
<p class="data-cell">{formatAmount(budget.totalSpent,{ currency: appStore.settings.currencyCode })}</p>
<p class="data-cell">{formatAmount(budget.remaining,{ currency: appStore.settings.currencyCode })}</p>
<style>
    :global(.delete-btn) {
        position: absolute;
        left: -24px;
        opacity: 0;

        &:hover {
            opacity: 1;
        }
    }
</style>
