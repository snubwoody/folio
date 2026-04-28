<script lang="ts">
    import { accountStore } from "$lib/stores/account.svelte";
    import { formatMoney } from "$lib/utils/money";
    import type { Account } from "$lib/types";

    type Props = {
        account: Account
    }

    const { account }: Props = $props();


    // TODO: test that account balance changes when handling transactions
    // TODO: change icon based on open state
    let name = $state("My account");
    let startingBalance = $state("0.00");
</script>

<!--FIXME: use accountBalance method-->
<li>
    <p class="text-truncate max-w-[50%]" title={account.name}>{account.name}</p>
    <p>{formatMoney(account.balance)}</p>
</li>

<style>
    li{
        display: flex;
        justify-content: space-between;
    }
</style>
