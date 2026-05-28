<script lang="ts">
    import { accountBalance, accountStore } from "$lib/stores/account.svelte";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { formatMoney } from "$lib/utils/money";

    type Props = {
        accountId: string;
    };

    const { accountId }: Props = $props();

    const account = $derived(accountStore.accountMap.get(accountId)!);
    const balance = $derived(
        accountBalance(account.id, transactionStore.transactions),
    );
</script>

<section>
    <h5>{account.name}</h5>
    <p>{formatMoney(balance.toString())}</p>
</section>

<style>
    section {
        padding: 24px;
    }
</style>