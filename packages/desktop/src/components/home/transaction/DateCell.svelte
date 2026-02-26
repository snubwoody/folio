<script lang="ts">
    import {type Transaction, transactionStore} from "$lib/transaction.svelte";
    import {Popover} from "bits-ui";
    import {formatDate} from "$lib/lib";
    import Calendar from "$components/Calendar.svelte";
    import { parseDate } from "@internationalized/date";


    interface Props{
        transaction: Transaction
    }

    const {transaction}: Props = $props();

    let displayDate = $state(formatDate(transaction.transactionDate));

    const updateDate = (year:number,month:number,day:number) => {
        transactionStore.editTransaction({ id: transaction.id,transactionDate: `${year}-${month}-${day}` });
        displayDate = formatDate(`${year}-${month}-${day}`)
    }
</script>

<td data-col="date">
    <Popover.Root>
        <Popover.Trigger>
            <time datetime={transaction.transactionDate}>
                {displayDate}
            </time>
        </Popover.Trigger>
        <Popover.Portal>
            <Popover.Content>
                <Calendar initialValue={parseDate(transaction.transactionDate)} onDateChange={updateDate}/>
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