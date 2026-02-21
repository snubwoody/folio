<script lang="ts">
    import { SelectCell } from "$components/table";
    import { accountStore } from "$lib/account.svelte";
    import { categoryStore } from "$lib/categories.svelte";
    import { formatAmount, formatDate } from "$lib/lib";
    import type { TableStore } from "$lib/stores/table.svelte";
    import { transactionStore, type Transaction } from "$lib/transaction.svelte";
    import { getContext } from "svelte";
    // TODO: make the row a form

    interface Props {
        transaction: Transaction
    }

    type TransactionType = "Expense" | "Income" | "Transfer";

    const { transaction }: Props = $props();

    const tableStore: TableStore = getContext("tableStore");
    tableStore.toggleSelect(transaction.id);

    let transactionType = $state<TransactionType>("Expense");
    if (transaction.fromAccountId && transaction.toAccountId){
        transactionType = "Transfer";
    } else if (transaction.toAccountId !== undefined && transaction.fromAccountId === undefined){
        transactionType = "Income";
    }

    const account = $derived(accountStore.accountMap.get(transaction.fromAccountId!));
    const category = $derived(categoryStore.categoryMap.get(transaction.categoryId??""));
    // TODO: make the row a form
    // TODO: add checkbox for selection
	// TODO:
    // - edit date
    // - edit amount
    // - edit note
    //   - add x button to clear
    let note = $state(transaction.note);
    let date = $state(formatDate(transaction.transactionDate));
    let selected = $derived(tableStore.isSelected(transaction.id));
    console.log(tableStore.isSelected(transaction.id));
</script>

<tr data-selected={selected}>
    <td >
        <input 
            checked={selected}
            type="checkbox" name="" id="" 
            onclick={(e)=>{
                if (!e.isTrusted) return
                console.log("Changed")
                if(e.currentTarget.checked){
                    tableStore.select(transaction.id)
                    console.log("Changed")
                    return
                }
                tableStore.deselect(transaction.id)
            }}
        >
    </td>
    <td>
        <!--TODO: parse dates-->
        <!--TODO: Add calendar below-->
        <input
            class="note-input"
            type="text"
            bind:value={date}
            onblur={() => transactionStore.editTransaction({ id: transaction.id,transactionDate: date })}
        >
    </td>
    <td>
        {#if transaction.fromAccountId !== undefined}
            <SelectCell
                value={account?.id}
                onChange={(id) => transactionStore.editTransaction({ id: transaction.id,fromAccountId: id })}
                items={accountStore.accounts.map(a => ({ value: a.id, label: a.name }))}
            />
        {/if}
    </td>
    <td>
        {#if transaction.toAccountId !== undefined}
            {@const payee = accountStore.accountMap.get(transaction.toAccountId)}
            {payee?.name}
        {/if}
    </td>
    <td>
        <input
            class="note-input"
            type="text"
            bind:value={note}
            onblur={() => transactionStore.editTransaction({ id: transaction.id,note: note })}
        >
    </td>
    <td>
        {#if transaction.categoryId !== undefined}
            <SelectCell
                value={category?.id}
                onChange={(id) => transactionStore.editTransaction({ id: transaction.id,categoryId: id })}
                items={categoryStore.categories.map(a => ({ value: a.id, label: a.title }))}
            />
        {/if}
    </td>
    <td>
        {#if transactionType === "Expense" }
            {formatAmount(transaction.amount)}
        {/if}
    </td>
    <td>
        {#if transactionType === "Income" }
            ${transaction.amount}
        {/if}
    </td>
</tr>

<style>
    .note-input{
        outline: none;
    }

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

        &[data-selected="true"]{
            background: var(--color-purple-50);
            border-color: black;
        }
    }
</style>