<script lang="ts">
    import SelectCell  from "./SelectCell.svelte";
    import { accountStore } from "$lib/account.svelte.js";
    import { categoryStore } from "$lib/categories.svelte.js";
    import { formatAmountWithoutSymbol, getCurrencySymbol } from "$lib/lib";
    import type { TableStore } from "$lib/stores/table.svelte.js";
    import { transactionStore, type Transaction,transactionType } from "$lib/transaction.svelte.js";
    import { appStore } from "$lib/state.svelte.js";
    import DateCell from "$components/home/transaction/DateCell.svelte";
    import {settingsStore} from "$lib/stores/settings.svelte";

    interface Props {
        transaction: Transaction,
        tableStore: TableStore
    }

    const { transaction,tableStore }: Props = $props();

    const transType = $derived.by(() => {
        return transactionType(transaction);
    });

    const category = $derived(categoryStore.categoryMap.get(transaction.categoryId??""));
    // TODO: make the row a form
    let note = $state(transaction.note);
    let selected = $derived(tableStore.isSelected(transaction.id));
    const currencySymbol = $derived(getCurrencySymbol(settingsStore.settings.currencyCode));
    const payeeOptions = $derived(accountStore.accounts.filter(a => a.id !== transaction.fromAccountId && a.id !== transaction.toAccountId));
    // TODO: clear money fields if there was an error parsing or reset
    // TODO: add set_account command instead
    // FIXME: inflow causing overflow
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
    <DateCell {transaction}/>
    <td data-col="account" data-testid="account">
        {#if transType === "Income"}
            {@const account = accountStore.accountMap.get(transaction.toAccountId??"")}
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
        {#if transType === "Transfer"}
            {@const account = accountStore.accountMap.get(transaction.toAccountId??"")}
            <SelectCell
                value={account?.id}
                onChange={(id) => transactionStore.setPayee({ id: transaction.id,accountId: id })}
                items={accountStore.accounts.map(a => ({ value: a.id, label: a.name }))}
            />
        {:else}
            <SelectCell
                onChange={(id) => transactionStore.setPayee({ id: transaction.id,accountId: id })}
                items={payeeOptions.map(a => ({ value: a.id, label: a.name }))}
            />
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
    <td data-col="outflow" data-testid="outflow">
        <div class="flex gap-1">
            {#if transType !== "Income"}
                <p>
                    {currencySymbol}
                </p>
                <input
                    type="text"
                    value={formatAmountWithoutSymbol(transaction.amount)}
                    class="outline-none"
                    onblur={(e) => transactionStore.setOutflow({ id:transaction.id,amount:e.currentTarget.value })}
                >
            {:else}
                <input
                    type="text"
                    class="outline-none"
                    onblur={(e) => transactionStore.setOutflow({ id:transaction.id,amount:e.currentTarget.value })}
                >
            {/if}
        </div>
    </td>
    <td data-col="inflow" data-testid="inflow">
        <div class="flex gap-1">
            {#if transType === "Income" }
                <p>
                    {currencySymbol}
                </p>
                <input
                    type="text"
                    value={formatAmountWithoutSymbol(transaction.amount)}
                    class="outline-none"
                    onblur={(e) => transactionStore.setInflow({ id:transaction.id,amount:e.currentTarget.value })}
                >
            {:else}
                <input
                    type="text"
                    class="outline-none"
                    onblur={(e) => transactionStore.setInflow({ id:transaction.id,amount:e.currentTarget.value })}
                >
            {/if}
        </div>
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
