<script lang="ts">
    import InlineTextField from "$components/InlineTextField.svelte";
    import {transactionStore} from "$lib/transaction.svelte";
    // TODO: make the row a form

    interface Transaction{
        id: string,
        amount: string
        from_account_id?: string
        to_account_id?: string
        category_id?: string
        note?: string
        date: string
    };

    $inspect(transactionStore.transactions);
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
            <tr>
                <td>
                    <InlineTextField value={transaction.transaction_date}/>
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
        {/each}
    </tbody>
</table>

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