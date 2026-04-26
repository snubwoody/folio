<script lang="ts">
    import { settingsStore } from "$lib/stores/settings.svelte";
    import type { Account } from "$lib/types";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { accountBalance } from "$lib/stores/account.svelte";
    import { formatMoney } from "$lib/utils/money";

    interface Props {
        account: Account
    }

    const { account }:Props = $props();
    const balance = $derived(accountBalance(account.id,transactionStore.transactions));
</script>

<li class="shadow-purple-sm p-2 rounded-md">
	<p>{account.name}</p>
	<h6>{formatMoney(balance.toString())}</h6>
</li>