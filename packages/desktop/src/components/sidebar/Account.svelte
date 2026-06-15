<script lang="ts">
    import { accountBalance } from "$lib/stores/account.svelte";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import type { Account } from "$lib/types";
    import { formatMoney } from "$lib/utils/money";

    type Props = {
        account: Account;
    };

    const { account }: Props = $props();

    const balance = $derived(
        accountBalance(account.id, transactionStore.transactions),
    );
</script>

{#key transactionStore.transactions}
    <li>
        <a href={`/accounts/${account.id}`}>
            <p class="text-truncate max-w-[50%]" title={account.name}>{account.name}</p>
            <p>{formatMoney(balance.toString())}</p>
        </a>
    </li>
{/key}

<style>
    a{
        display: flex;
        justify-content: space-between;
    }
</style>
