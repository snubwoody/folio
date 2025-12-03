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
    import {
        formatAmountWithoutSymbol,
        formatDate,
        getCurrencySymbol,
        type Expense,
        type Category,
        type Account
    } from "$lib/lib";
    import { appStore } from "$lib/state.svelte";
    import DatePicker from "../../DatePicker.svelte";
    import MoneyCell from "$components/MoneyCell.svelte";
    import SelectButton from "$components/SelectButton.svelte";
    import IconButton from "$components/button/IconButton.svelte";
    import { Trash2 } from "@lucide/svelte";
    import {TableCell} from "$components/table";

    type Props = {
        expense: Expense;
    };

    const { expense }: Props = $props();
    const symbol = $derived(getCurrencySymbol(appStore.settings.currencyCode));
    const formattedAmount = $derived.by(() =>
        formatAmountWithoutSymbol(expense.amount, {
            currency: appStore.settings.currencyCode
        })
    );

    function updateDate(year: number, month: number, day: number) {
        appStore.transactions.editExpense({
            id: expense.id,
            date: `${year}-${month}-${day}`
        });
    }

    async function updateAmount(amount: string) {
        await appStore.transactions.editExpense({ id: expense.id, amount });
    }

    async function editCategory(item: Category) {
        await appStore.transactions.editExpense({
            id: expense.id,
            categoryId: item.id
        });
    }

    async function editAccount(item: Account) {
        await appStore.transactions.editExpense({
            id: expense.id,
            accountId: item.id
        });
    }
</script>

<TableCell class="justify-between relative">
    <p>{expense.category?.title ?? " "}</p>
    <IconButton
        aria-label="Delete expense"
        class="absolute -left-3 opacity-0 hover:opacity-100"
        size="small"
        variant="ghost"
        onclick={() => appStore.transactions.deleteExpense(expense.id)}
    >
        <Trash2 />
    </IconButton>
    <SelectButton
        items={appStore.categories}
        toOption={(c) => {
            return { label: c.title, value: c.id };
        }}
        onChange={editCategory}
    />
</TableCell>
<TableCell class="justify-between">
    <p>{expense.account?.name ?? " "}</p>
    <SelectButton
        items={appStore.accounts}
        toOption={(a) => {
            return { label: a.name, value: a.id };
        }}
        onChange={editAccount}
    />
</TableCell>
<TableCell class="justify-between">
    <p>{formatDate(expense.date)}</p>
    <DatePicker onDateChange={updateDate} />
</TableCell>
<MoneyCell {symbol} amount={formattedAmount} onUpdate={updateAmount} />
