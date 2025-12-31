<script lang="ts">
    import { appStore } from "$lib/state.svelte";
    import type { DataCellParams, DataColumn, DataRow } from "$lib/table";
    import { SelectCell, Table, TableCell, TableHeader } from "$components/table";
    import { formatAmountWithoutSymbol, formatDate, getCurrencySymbol } from "$lib/lib";
    import DatePicker from "$components/DatePicker.svelte";
    import MoneyCell from "$components/MoneyCell.svelte";

    const symbol = $derived(getCurrencySymbol(appStore.settings.currencyCode));

    const columns: DataColumn[] = [
        { id: "Income stream" },
        { id: "Account" },
        { id: "Date" },
        { id: "Amount" }
    ];

    const rows: DataRow[] = $derived.by(() =>
        appStore.incomes.map(income => {
            {return { id: income.id };}
        })
    );

    const cells = $derived.by(() => {
        const cells: DataCellParams[] = [];
        appStore.incomes.forEach(income => {
            // FIXME allow null values
            cells.push({ value: income.incomeStream?.id ?? "" });
            cells.push({ value: income.account?.id ?? "" });
            cells.push({ value: income.date });
            cells.push({ value: income.amount });
        });
        return cells;
    });

    const incomeStreams = $derived.by(() =>
        appStore.incomeStreams.map(incomeStream => {
            return { value: incomeStream.id,label: incomeStream.title };
        })
    );
    const accounts = $derived.by(() =>
        appStore.accounts.map(account => {
            return { value: account.id,label: account.name };
        })
    );

    async function editIncomeStream(incomeId: string,incomeStreamId: string) {
        await appStore.transactions.editIncome({
            id: incomeId,
            incomeStreamId
        });
    }

    async function editAccount(incomeId: string, accountId: string) {
        await appStore.transactions.editIncome({
            id: incomeId,
            accountId
        });
    }

    function updateDate(incomeId: string,year: number, month: number, day: number) {
        appStore.transactions.editIncome({
            id: incomeId,
            date: `${year}-${month}-${day}`
        });
    }

    async function updateAmount(amount: string,id: string) {
        await appStore.transactions.editIncome({ id, amount });
    }
</script>

<Table aria-label="Income table" {cells} {columns} {rows}>
    {#snippet header(label)}
        <TableHeader>{label}</TableHeader>
    {/snippet}
    {#snippet cell({ value,columnId,rowId })}
        {#if columnId === "Income stream"}
            <SelectCell
                {value}
                onChange={(value) => editIncomeStream(rowId,value)}
                items={incomeStreams}
            />
        {:else if columnId === "Account"}
            <SelectCell
                {value}
                onChange={(value) => editAccount(rowId,value)}
                items={accounts}
            />
        {:else if columnId === "Date"}
            <TableCell>
                <div class="flex justify-between items-center">
                    <p>{formatDate(value)}</p>
                    <DatePicker onDateChange={(year,month,day) => updateDate(rowId,year,month,day)} />
                </div>
            </TableCell>
        {:else}
            {@const formattedAmount = formatAmountWithoutSymbol(value, {
                currency: appStore.settings.currencyCode
            })}
            <MoneyCell
                {symbol}
                amount={formattedAmount}
                onUpdate={(value) => updateAmount(value,rowId)}
            />
        {/if}
    {/snippet}
</Table>

