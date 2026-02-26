<script lang="ts">
    import { SelectCell } from "$components/table";
    import { accountStore } from "$lib/account.svelte";
    import { categoryStore } from "$lib/categories.svelte";
    import {formatAmount, formatAmountWithoutSymbol, formatDate, getCurrencySymbol} from "$lib/lib";
    import type { TableStore } from "$lib/stores/table.svelte";
    import { transactionStore, type Transaction } from "$lib/transaction.svelte";
    import {appStore} from "$lib/state.svelte";

    interface Props {
        transaction: Transaction,
        tableStore: TableStore
    }

    const { transaction,tableStore }: Props = $props();

    const isIncome = transaction.toAccountId !== null && transaction.fromAccountId === null;
    // const isExpense = transaction.toAccountId === null && transaction.fromAccountId !== null;
    const isTransfer = transaction.toAccountId !== null && transaction.fromAccountId !== null;

    const category = $derived(categoryStore.categoryMap.get(transaction.categoryId??""));
    // TODO: make the row a form
	// TODO:
    // - edit date
    // - edit amount
    // - edit note
    //   - add x button to clear
    let note = $state(transaction.note);
    let date = $state(formatDate(transaction.transactionDate));
    let selected = $derived(tableStore.isSelected(transaction.id));
    const currencySymbol = $derived(getCurrencySymbol(appStore.settings.currencyCode));
</script>

<tr data-selected={selected}>
    <td data-col="checkbox">
        <input
            checked={tableStore.isSelected(transaction.id)}
            type="checkbox" name="" id=""
            onclick={(e) => {
                if (!e.isTrusted) return;
                if(e.currentTarget.checked){
                    tableStore.select(transaction.id);
                    return;
                }
                tableStore.deselect(transaction.id);
            }}
        >
    </td>
    <td data-col="date">
        <!--TODO: parse dates-->
        <!--TODO: Add calendar below-->
        <!--TODO: use <time> tag-->
        <input
            class="note-input"
            type="text"
            bind:value={date}
            onblur={() => transactionStore.editTransaction({ id: transaction.id,transactionDate: date })}
        >
    </td>
    <td data-col="account" data-testid="account">
        {#if isIncome}
            {@const account = accountStore.accountMap.get(transaction.fromAccountId??"")}
            <SelectCell
                value={account?.id}
                onChange={(id) => transactionStore.editTransaction({ id: transaction.id,toAccountId: id })}
                items={accountStore.accounts.map(a => ({ value: a.id, label: a.name }))}
            />
        {:else}
            {@const account = accountStore.accountMap.get(transaction.fromAccountId??"")}
            <SelectCell
                value={account?.id}
                onChange={(id) => transactionStore.editTransaction({ id: transaction.id,fromAccountId: id })}
                items={accountStore.accounts.map(a => ({ value: a.id, label: a.name }))}
            />
        {/if}
    </td>
    <td data-col="payee" data-testid="payee">
        {#if isTransfer}
            {@const payee = accountStore.accountMap.get(transaction.toAccountId??"")}
            {payee?.name}
        {/if}
    </td>
    <td data-col="note" data-testid="note">
        <input
            class="note-input"
            type="text"
            bind:value={note}
            onblur={() => transactionStore.editTransaction({ id: transaction.id,note: note })}
        >
    </td>
    <td data-col="category" data-testid="category">
        {#if transaction.categoryId !== undefined}
            <SelectCell
                value={category?.id}
                onChange={(id) => transactionStore.editTransaction({ id: transaction.id,categoryId: id })}
                items={categoryStore.categories.map(a => ({ value: a.id, label: a.title }))}
            />
        {/if}
    </td>
    <td data-col="outflow" data-testid="outflow" class="flex gap-1">
        {#if transaction.fromAccountId}
            <p>
                {currencySymbol}
            </p>
            <input
                type="text"
                value={formatAmountWithoutSymbol(transaction.amount)}
                class="outline-none"
                onblur={(e)=>transactionStore.setOutflow({id:transaction.id,amount:e.currentTarget.value})}
            >
        {/if}
    </td>
    <td data-col="inflow" data-testid="inflow">
        {#if transaction.toAccountId }
            {formatAmount(transaction.amount)}
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