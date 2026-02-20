<script lang="ts">
    import InlineTextField from "$components/InlineTextField.svelte";
    import { formatDate } from "$lib/lib";
    import {transactionStore} from "$lib/transaction.svelte";
    import Transaction from "./Transaction.svelte";
</script>

<table>
    <thead>
        <tr>
            <th>Date</th>
            <th>Account</th>
            <th>Payee</th>
            <th>Note</th>
            <th>Category</th>
            <th>Outflow</th>
            <th>Inflow</th>
        </tr>
    </thead>
    <tbody>
        {#each transactionStore.transactions as transaction (transaction.id)}
            <Transaction {transaction}/>
        {/each}
    </tbody>
</table>

<style>
    table{
        table-layout: fixed;
        width: 100%;
    }

    th{
        text-align: left;

        &:last-child{
            text-align: right;
        }

        padding: 8px 16px;
        border: 1px solid var(--color-neutral-50);
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