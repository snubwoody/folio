<script lang="ts">
    import InlineTextField from "$components/InlineTextField.svelte";
    import { formatDate } from "$lib/lib";
    import {transactionStore,type Transaction} from "$lib/transaction.svelte";
    // TODO: make the row a form

    interface Props {
        transaction: Transaction
    }

    type TransactionType = "Expense" | "Income" | "Transfer"

    const {transaction}: Props = $props();
    // TODO: make the row a form
</script>

<tr>
    <td>
        <InlineTextField value={formatDate(transaction.transaction_date)}/>
    </td>
    <td>
        {#if transaction.from_account_id !== undefined}
            Account
        {/if}
    </td>
    <td>
        {#if transaction.to_account_id !== undefined}
            Payee
        {/if}
    </td>
    <td>{transaction.note}</td>
    <td>{transaction.category_id}</td>
    <td>${transaction.amount}</td>
    <td>${transaction.amount}</td>
</tr>


<style>
    table{
        table-layout: fixed;
        width: 100%;
    }

    td,th{
        text-align: left;

        &:last-child{
            text-align: right;
        }

        padding: 8px 16px;
    }

    thead{
        background: var(--color-neutral-25);
    }

    tr{
        border-bottom: 1px solid var(--color-neutral-50);

        &:first-child{
            border-top: 1px solid var(--color-neutral-50);
        }
    }
</style>