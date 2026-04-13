<script lang="ts">
    import { SelectCell }  from "$components/table";
    import { accountStore } from "$lib/stores/account.svelte";
    import { categoryStore } from "$lib/stores/categories.svelte";
    import type { TableStore } from "$lib/stores/table.svelte.js";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { type Transaction, transactionType } from "$lib/api/transaction";
    import DateCell from "./DateCell.svelte";
    import { TableCell } from "$components/table";
    import AccountCell from "./AccountCell.svelte";
    import { settingsStore } from "$lib/stores/settings.svelte";
    import { TableRow } from "$components/table";
    import {formatAmountWithoutSymbol, getCurrencySymbol} from "$lib/utils/money";

    interface Props{
        transaction: Transaction
        tableStore: TableStore
    }

    const { transaction,tableStore }: Props = $props();
    const transType = $derived.by(() => {
        return transactionType(transaction);
    });

    const category = $derived(categoryStore.categoryMap.get(transaction.categoryId??""));
    // TODO: make the row a form
    let note = $derived(transaction.note);
    let selected = $derived(tableStore.isSelected(transaction.id));
    const currencySymbol = $derived(getCurrencySymbol(settingsStore.settings.currencyCode));
    const payeeOptions = $derived(accountStore.accounts.filter(a => a.id !== transaction.fromAccountId && a.id !== transaction.toAccountId));
    // TODO: test this
    // FIXME: popup width for select without a selected option
    // FIXME: only update payee if the function succeeds
    // TODO: add style for selected items
    // FIXME: make calendar and popup fit cells
</script>

<TableRow data-selected={selected}>
    <TableCell class="checkbox-cell">
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
    </TableCell>
    <DateCell {transaction}/>
    <AccountCell {transaction}/>
    {#if transType === "Transfer"}
        {@const account = accountStore.accountMap.get(transaction.toAccountId??"")}
        <SelectCell
            data-testid="payee"
            value={account?.id}
            onChange={(accountId) => transactionStore.setPayee(transaction.id,accountId)}
            items={accountStore.accounts.map(a => ({ value: a.id, label: a.name }))}
        />
    {:else}
        <SelectCell
            data-testid="payee"
            onChange={(accountId) => transactionStore.setPayee(transaction.id,accountId)}
            items={payeeOptions.map(a => ({ value: a.id, label: a.name }))}
        />
    {/if}
    <TableCell>
        <input
            class="note-input outline-none"
            type="text"
            bind:value={note}
            onblur={() => transactionStore.editTransaction({ id: transaction.id,note: note })}
        >
    </TableCell>
    <SelectCell
        data-testid="category"
        value={category?.id}
        onChange={(id) => transactionStore.editTransaction({ id: transaction.id,categoryId: id })}
        items={categoryStore.allCategories.map(a => ({ value: a.id, label: a.title }))}
    />
    <TableCell data-testid="outflow" class="flex gap-1 items-center">
        <!--TODO: kind of unnecessary-->
        {#if transType !== "Income"}
            <p>
                {currencySymbol}
            </p>
            <input
                type="text"
                value={formatAmountWithoutSymbol(transaction.amount)}
                class="outline-none"
                onblur={(e) => transactionStore.setOutflow(transaction.id,e.currentTarget.value)}
            >
        {:else}
            <input
                type="text"
                class="outline-none"
                onblur={(e) => transactionStore.setOutflow(transaction.id,e.currentTarget.value)}
            >
        {/if}
    </TableCell>
    <TableCell data-testid="inflow" class="flex gap-1 items-center">
        {#if transType === "Income" }
            <p>
                {currencySymbol}
            </p>
            <input
                type="text"
                value={formatAmountWithoutSymbol(transaction.amount)}
                class="outline-none"
                onblur={(e) => transactionStore.setInflow(transaction.id,e.currentTarget.value)}
            >
        {:else}
            <input
                type="text"
                class="outline-none"
                onblur={(e) => transactionStore.setInflow(transaction.id,e.currentTarget.value)}
            >
        {/if}
    </TableCell>
</TableRow>