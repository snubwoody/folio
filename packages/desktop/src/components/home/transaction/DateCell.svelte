<script lang="ts">
    import { type Transaction, transactionStore } from "$lib/stores/transaction.svelte";
    import { Popover } from "bits-ui";
    import { formatDate } from "$lib/lib";
    import Calendar from "$components/Calendar.svelte";
    import { parseDate, type DateValue } from "@internationalized/date";

    interface Props{
        transaction: Transaction
    }

    const { transaction }: Props = $props();

    let displayDate = $state(formatDate(transaction.transactionDate));

    const updateDate = async (date: DateValue) => {
        calendarOpen = false;
        await transactionStore.editTransaction({ id: transaction.id,transactionDate: `${date.year}-${date.month}-${date.day}` });
        displayDate = formatDate(`${date.year}-${date.month}-${date.day}`);
    };
    let date = $state(parseDate(transaction.transactionDate));
    let calendarOpen = $state(false);
</script>

<td data-col="date">
    <Popover.Root bind:open={calendarOpen}>
        <Popover.Trigger>
            <time datetime={transaction.transactionDate}>
                {displayDate}
            </time>
        </Popover.Trigger>
        <Popover.Portal>
            <Popover.Content>
                <Calendar bind:value={date} onDateChange={updateDate}/>
            </Popover.Content>
        </Popover.Portal>
    </Popover.Root>
</td>

<style>
    td{
        text-align: left;

        &:last-child{
            text-align: right;
        }

        padding: 8px 16px;
        border: 1px solid var(--color-neutral-50);

        &:focus-within{
            background: var(--color-purple-50);
            border-color: var(--color-purple-500);
        }
    }

</style>