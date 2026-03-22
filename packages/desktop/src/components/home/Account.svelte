<script lang="ts">
    import { formatAmount } from "$lib/lib";
    import { settingsStore } from "$lib/stores/settings.svelte";
    import type { Account } from "$lib/lib";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { accountBalance } from "$lib/account.svelte";

    interface Props {
        account: Account
    }

    const { account }:Props = $props();
    const balance = $derived(accountBalance(account.id,transactionStore.transactions));
</script>

<li class="shadow-purple-sm p-2 rounded-md">
	<p>{account.name}</p>
	<h6>{formatAmount(balance.toString(),{ currency: settingsStore.settings.currencyCode })}</h6>
</li>