<script lang="ts">
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { Popover } from "bits-ui";
    import Calendar from "$components/Calendar.svelte";
    import {type DateValue, toCalendarDate} from "@internationalized/date";
    import type { Transaction } from "$lib/api/transaction";
    import { TableCell } from "$components/table";
    import {formatDate} from "$lib/utils/date";

    interface Props{
        transaction: Transaction
    }

    const { transaction }: Props = $props();

    let displayDate = $derived(formatDate(transaction.date));

    // TODO: make editTransaction take date
    const updateDate = async (date: DateValue) => {
        calendarOpen = false;

        await transactionStore.editTransaction({ id: transaction.id,transactionDate: date.toString() });
        displayDate = formatDate(toCalendarDate(date));
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
