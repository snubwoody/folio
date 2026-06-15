<script lang="ts">
    import { type DateValue, toCalendarDate } from "@internationalized/date";
    import { Popover } from "bits-ui";
    import { Calendar } from "$components/date";
    import { TableCell } from "$components/table";
    import type { Transaction } from "$lib/api/transaction";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { formatDate } from "$lib/utils/date";
    import { settingsStore } from "$lib/stores/settings.svelte";
    import { formatAmountWithoutSymbol, parseMoney } from "$lib/utils/money";

    interface Props {
        transaction: Transaction;
        value?: string;
        onSubmit: (amount: string) => void;
    }

    const { transaction, value }: Props = $props();

    let displayDate = $derived(formatDate(transaction.date));

    const submit = (value: string) => {
        const amount = parseMoney(value);
        if (amount){

        }
    }

    const updateDate = async (date: DateValue) => {
        calendarOpen = false;

        await transactionStore.editTransaction({
            id: transaction.id,
            transactionDate: date.toString(),
        });
        displayDate = formatDate(toCalendarDate(date));
    };
    let date = $derived(transaction.date);
    let amount = $state(value);
    let calendarOpen = $state(false);

    const currencySymbol = $derived(settingsStore.currencySymbol);
</script>

<!-- {#if transType === "Income" }
    <p>
        {currencySymbol}
    </p>
    <input
        type="text"
        value={formatAmountWithoutSymbol(transaction.amount)}
        class="outline-none"
        onblur={(e) => transactionStore.setInflow(transaction.id,e.currentTarget.value)}
    >
{:else}
    <input
        type="text"
        class="outline-none"
        onblur={(e) => transactionStore.setInflow(transaction.id,e.currentTarget.value)}
    >
{/if} -->

<TableCell>
    <Popover.Root bind:open={calendarOpen}>
        <Popover.Trigger class="text-left">
            <time datetime={date.toString()}>
                {displayDate}
            </time>
        </Popover.Trigger>
        <Popover.Portal>
            <Popover.Content
                onCloseAutoFocus={(e) => e.preventDefault()}
                onInteractOutside={() => updateDate(date)}>
                    {#if value}

                    {/if}
                    <p>
                        {currencySymbol}
                    </p>
                    <input
                        type="text"
                        value={formatAmountWithoutSymbol(transaction.amount)}
                        class="outline-none"
                        onblur={(e) => submit(e.currentTarget.value)}
                    >
                <Calendar bind:value={date} onDateChange={updateDate}/>
            </Popover.Content>
        </Popover.Portal>
    </Popover.Root>
</TableCell>
