<script lang="ts">
    import SelectCell  from "./SelectCell.svelte";
    import { accountStore } from "$lib/stores/account.svelte";
    import { categoryStore } from "$lib/stores/categories.svelte";
    import { formatAmountWithoutSymbol, getCurrencySymbol } from "$lib/lib";
    import type { TableStore } from "$lib/stores/table.svelte.js";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { type Transaction, transactionType } from "$lib/transaction";
    import DateCell from "$components/table/DateCell.svelte";
    import AccountCell from "$components/table/AccountCell.svelte";
    import { settingsStore } from "$lib/stores/settings.svelte";
    import {TableRow} from "$components/table";

    interface Props{
        transaction: Transaction
    }

    const {transaction}: Props = $props();
    const transType = $derived.by(() => {
        return transactionType(transaction);
    });

    const category = $derived(categoryStore.categoryMap.get(transaction.categoryId??""));
    // TODO: make the row a form
    let note = $derived(transaction.note);
    let selected = $derived(tableStore.isSelected(transaction.id));
    const currencySymbol = $derived(getCurrencySymbol(settingsStore.settings.currencyCode));
    const payeeOptions = $derived(accountStore.accounts.filter(a => a.id !== transaction.fromAccountId && a.id !== transaction.toAccountId));
    // TODO: clear money fields if there was an error parsing or reset
    // TODO: add set_account command instead
    // FIXME: inflow causing overflow
    // TODO: switch to divs or ul
</script>

<TableRow>
    <DateCell {transaction}/>
    <AccountCell {transaction}/>
    <div data-testid="payee" class="t-cell">
        {#if transType === "Transfer"}
            {@const account = accountStore.accountMap.get(transaction.toAccountId??"")}
            <SelectCell
                value={account?.id}
                onChange={(accountId) => transactionStore.setPayee(transaction.id,accountId)}
                items={accountStore.accounts.map(a => ({ value: a.id, label: a.name }))}
            />
        {:else}
            <SelectCell
                onChange={(accountId) => transactionStore.setPayee(transaction.id,accountId)}
                items={payeeOptions.map(a => ({ value: a.id, label: a.name }))}
            />
        {/if}
    </div>
    <div class="t-cell">Note</div>
    <div class="t-cell">Category</div>
    <div class="t-cell">Outflow</div>
    <div class="t-cell">Inflow</div>
</TableRow>