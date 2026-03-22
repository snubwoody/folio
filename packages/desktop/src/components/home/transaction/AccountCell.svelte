<script lang="ts">
    import SelectCell  from "./SelectCell.svelte";
    import { accountStore } from "$lib/account.svelte";
    import { transactionStore, type Transaction } from "$lib/transaction.svelte";

    interface Props {
        transaction: Transaction
    }

    const {transaction}: Props = $props();
    const account = $derived(accountStore.accountMap.get(transaction.fromAccountId??""));
</script>

<td data-col="account" data-testid="account" class="table-cell">
    <SelectCell
        value={account?.id}
        onChange={(id) => transactionStore.editTransaction({ id: transaction.id,fromAccountId: id })}
        items={accountStore.accounts.map(a => ({ value: a.id, label: a.name }))}
    />
</td>
<!-- <td data-col="account" data-testid="account">
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
</td> -->