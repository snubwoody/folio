<script lang="ts">
    import { SelectCell } from "$components/table";
    import { accountStore } from "$lib/stores/account.svelte";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { transactionType, type Transaction } from "$lib/transaction";

    interface Props {
        transaction: Transaction
    }

    const { transaction }: Props = $props();

    const transType = $derived.by(() => {
        return transactionType(transaction);
    });

    const fromAccount = $derived(accountStore.accountMap.get(transaction.fromAccountId??""));
    const toAccount = $derived(accountStore.accountMap.get(transaction.toAccountId??""));
    // TODO: add custom UI for disabled payee items
</script>

{#if transType === "Income"}
    <SelectCell
        data-testid="account"
        value={toAccount?.id}
        onChange={(id) => transactionStore.editTransaction({ id: transaction.id,toAccountId: id })}
        items={accountStore.accounts.map(a => ({ value: a.id, label: a.name }))}
    />
{:else}
    <SelectCell
        data-testid="account"
        value={fromAccount?.id}
        onChange={(id) => transactionStore.editTransaction({ id: transaction.id,fromAccountId: id })}
        items={accountStore.accounts.map(a => ({ value: a.id, label: a.name,disabled: a.id === transaction.toAccountId }))}
    />
{/if}
