<script lang="ts">
    import SelectCell  from "./SelectCell.svelte";
    import { accountStore } from "$lib/account.svelte";
    import { transactionStore, transactionType, type Transaction } from "$lib/transaction.svelte";

    interface Props {
        transaction: Transaction
    }

    const { transaction }: Props = $props();

    const transType = $derived.by(() => {
        return transactionType(transaction);
    });

    const fromAccount = $derived(accountStore.accountMap.get(transaction.fromAccountId??""));
    const toAccount = $derived(accountStore.accountMap.get(transaction.toAccountId??""));
</script>

<td data-col="account" data-testid="account" class="table-cell">
    {#if transType === "Income"}
        <SelectCell
            value={toAccount?.id}
            onChange={(id) => transactionStore.editTransaction({ id: transaction.id,toAccountId: id })}
            items={accountStore.accounts.map(a => ({ value: a.id, label: a.name }))}
        />
    {:else}
        <SelectCell
            value={fromAccount?.id}
            onChange={(id) => transactionStore.editTransaction({ id: transaction.id,fromAccountId: id })}
            items={accountStore.accounts.map(a => ({ value: a.id, label: a.name }))}
        />
    {/if}
</td>