<script lang="ts">
    import MoneyCell from "$components/MoneyCell.svelte";
    import {
        formatAmountWithoutSymbol,
        formatDate,
        getCurrencySymbol,
        type Income,
        type IncomeStream,
        type Account
    } from "$lib/lib";
    import { appStore } from "$lib/state.svelte";
    import DatePicker from "../../DatePicker.svelte";
    import IconButton from "$components/button/IconButton.svelte";
    import { Trash2 } from "@lucide/svelte";
    import SelectButton from "$components/SelectButton.svelte";

    type Props = {
        income: Income;
    };

    const { income }: Props = $props();
    const symbol = $derived(getCurrencySymbol(appStore.settings.currencyCode));
    const formattedAmount = $derived.by(() =>
        formatAmountWithoutSymbol(income.amount, {
            currency: appStore.settings.currencyCode
        })
    );

    function updateDate(year: number, month: number, day: number) {
        appStore.transactions.editIncome({
            id: income.id,
            date: `${year}-${month}-${day}`
        });
    }

    async function updateAmount(amount: string) {
        await appStore.transactions.editIncome({ id: income.id, amount });
    }

    async function editIncomeStream(item: IncomeStream) {
        await appStore.transactions.editIncome({
            id: income.id,
            incomeStreamId: item.id
        });
    }

    async function editAccount(item: Account) {
        await appStore.transactions.editIncome({
            id: income.id,
            accountId: item.id
        });
    }
</script>

<div class="data-cell flex justify-between items-center relative">
    <p>{income.incomeStream?.title ?? " "}</p>
    <IconButton
        class="delete-btn"
        size="small"
        variant="ghost"
        onclick={() => appStore.transactions.deleteIncome(income.id)}
    >
        <Trash2 />
    </IconButton>
    <SelectButton
        items={appStore.incomeStreams}
        toOption={(c) => {
            return { label: c.title, value: c.id };
        }}
        onChange={editIncomeStream}
    />
</div>
<div class="data-cell flex justify-between items-center">
    <p>{income.account?.name ?? " "}</p>
    <SelectButton
        items={appStore.accounts}
        toOption={(c) => {
            return { label: c.name, value: c.id };
        }}
        onChange={editAccount}
    />
</div>
<li class="data-cell flex items-center justify-between">
    <p>{formatDate(income.date)}</p>
    <DatePicker onDateChange={updateDate} />
</li>
<MoneyCell {symbol} amount={formattedAmount} onUpdate={updateAmount} />
