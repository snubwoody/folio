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

<div class="data-cell flex justify-between items-center relative">
    <p>{expense.category?.title ?? " "}</p>
    <IconButton
        class="delete-btn"
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
</div>
<div class="data-cell flex justify-between items-center">
    <p>{expense.account?.name ?? " "}</p>
    <SelectButton
        items={appStore.accounts}
        toOption={(a) => {
            return { label: a.name, value: a.id };
        }}
        onChange={editAccount}
    />
</div>
<li class="data-cell flex items-center justify-between">
    <p>{formatDate(expense.date)}</p>
    <DatePicker onDateChange={updateDate} />
</li>
<MoneyCell {symbol} amount={formattedAmount} onUpdate={updateAmount} />
