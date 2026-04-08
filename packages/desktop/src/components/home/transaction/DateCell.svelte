<script lang="ts">
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { Popover } from "bits-ui";
    import { formatDate } from "$lib/lib";
    import Calendar from "$components/Calendar.svelte";
    import type { DateValue } from "@internationalized/date";
    import type { Transaction } from "$lib/transaction";
    import { TableCell } from "$components/table";

    interface Props{
        transaction: Transaction
    }

    const { transaction }: Props = $props();

    let displayDate = $derived(formatDate(transaction.date.toString()));

    // FIXME: make editTransaction take date
    const updateDate = async (date: DateValue) => {
        calendarOpen = false;
        await transactionStore.editTransaction({ id: transaction.id,transactionDate: `${date.year}-${date.month}-${date.day}` });
        displayDate = formatDate(`${date.year}-${date.month}-${date.day}`);
    };
    let date = $derived(transaction.date);
    let calendarOpen = $state(false);
</script>

<TableCell>
    <Popover.Root bind:open={calendarOpen}>
        <Popover.Trigger class="text-left">
            <time datetime={transaction.date.toString()}>
                {displayDate}
            </time>
        </Popover.Trigger>
        <Popover.Portal>
            <Popover.Content>
                <Calendar bind:value={date} onDateChange={updateDate}/>
            </Popover.Content>
        </Popover.Portal>
    </Popover.Root>
</TableCell>

<style>
    /*td{*/
    /*    text-align: left;*/

    /*    &:last-child{*/
    /*        text-align: right;*/
    /*    }*/

    /*    padding: 8px 16px;*/
    /*    border: 1px solid var(--color-neutral-50);*/

    /*    &:focus-within{*/
    /*        background: var(--color-purple-50);*/
    /*        border-color: var(--color-purple-500);*/
    /*    }*/
    /*}*/

</style>