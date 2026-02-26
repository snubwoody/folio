<script lang="ts">
    import type {Transaction} from "$lib/transaction.svelte";
    import {formatDate} from "$lib/lib";

    interface Props{
        transaction: Transaction
    }

    const {transaction}: Props = $props();

    const displayDate = $state(formatDate(transaction.transactionDate));
</script>

<td data-col="date">
    <!--TODO: parse dates-->
    <!--TODO: Add calendar below-->
    <!--TODO: use <time> tag-->
    <!--        <input-->
    <!--            class="note-input"-->
    <!--            type="text"-->
    <!--            bind:value={date}-->
    <!--            onblur={() => transactionStore.editTransaction({ id: transaction.id,transactionDate: date })}-->
    <!--        >-->
        <time datetime={transaction.transactionDate}>
            {displayDate}
        </time>
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