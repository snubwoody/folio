<script lang="ts">
    import InlineTextField from "$components/InlineTextField.svelte";
    import { SelectCell } from "$components/table";
    import { accountStore } from "$lib/account.svelte";
    import { formatDate } from "$lib/lib";
    import {transactionStore,type Transaction} from "$lib/transaction.svelte";
    // TODO: make the row a form

    interface Props {
        transaction: Transaction
    }

    type TransactionType = "Expense" | "Income" | "Transfer"

    const {transaction}: Props = $props();
    const payee = accountStore.accountMap.get(transaction.toAccountId??"");
    const account = $derived(accountStore.accountMap.get(transaction.fromAccountId!));
    // TODO: make the row a form
    // TODO: add checkbox for selection
</script>

<tr>
    <td>
        <InlineTextField value={formatDate(transaction.transactionDate)}/>
    </td>
    <td>
        {#if transaction.fromAccountId !== undefined}
            <SelectCell
                value={account?.id}
                onChange={(value) => console.log(value)}
                items={accountStore.accounts.map(a => ({value: a.id, label: a.name}))}
            />
        {/if}
    </td>
    <td>
        {#if transaction.toAccountId !== undefined}
            {@const payee = accountStore.accountMap.get(transaction.toAccountId)}
            {payee?.name}
        {/if}
    </td>
    <td>{transaction.note}</td>
    <td>{transaction.categoryId}</td>
    <td>${transaction.amount}</td>
    <td>${transaction.amount}</td>
</tr>


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

    tr{
        border-bottom: 1px solid var(--color-neutral-50);

        &:first-child{
            border-top: 1px solid var(--color-neutral-50);
        }
    }
</style>