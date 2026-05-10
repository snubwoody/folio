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
    import { appStore } from "$lib/state.svelte";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { getLocalTimeZone, isSameMonth, today } from "@internationalized/date";
    import { transactionType } from "$lib/api/transaction";
    import { formatMoney } from "$lib/utils/money";

    const currentDate = today(getLocalTimeZone());
    const incomes = transactionStore.transactions.filter(transaction => {
        const transType = transactionType(transaction);
        return isSameMonth(transaction.date,currentDate) && transType === "Income";
    });
    const expenses = transactionStore.transactions.filter(transaction => {
        const transType = transactionType(transaction);
        return isSameMonth(transaction.date,currentDate) && transType === "Expense";
    });

    const budgets = appStore.budgets;
    // FIXME: broken state
    const totalExpenses = $derived.by(() => expenses.reduce((acc,item) => acc + parseFloat(item.amount),0));
    const totalIncome = $derived.by(() => incomes.reduce((acc,item) => acc + parseFloat(item.amount),0));
    const totalBudget = $derived.by(() => budgets.reduce((acc,item) => acc + parseFloat(item.amount),0));
    // TODO: format large values
    const percentage = $derived.by(() => (totalExpenses/totalIncome) * 100);
    // FIXME: nan percentage
</script>

<section class="flex items-center justify-between">
    <div>
        <h5>Monthly budget</h5>
        <h3>{formatMoney(totalBudget.toString(),{ compact: true })}</h3>
    </div>
    <div>
        <h5>Expenses</h5>
        <h3>{formatMoney(totalExpenses.toString(),{ compact: true })}</h3>
        {#if totalIncome > 0 && totalExpenses > 0}
            <p class="text-sm">{percentage.toFixed(0)}% of income</p>
        {/if}
    </div>
    <div>
        <h5>Income</h5>
        <h3>{formatMoney(totalIncome.toString(),{ compact: true })}</h3>
    </div>
</section>

<style>
    h5{
        margin-bottom: 20px;
    }

    h3{
        margin-bottom: 8px;
    }
</style>