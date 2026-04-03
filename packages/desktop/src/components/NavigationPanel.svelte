
<script>
    import IconButton from "$components/button/IconButton.svelte";
    import Home from "@lucide/svelte/icons/house";
    import Chart from "@lucide/svelte/icons/chart-no-axes-combined";
    import SettingsButton from "$components/settings/SettingsButton.svelte";
    import {Plus} from "@lucide/svelte";
    import {Button} from "$components/button";
    import {accountStore} from "$lib/stores/account.svelte";
    import {formatMoney} from "$lib/lib";
    // TODO: add aria-expanded

    const total = $derived.by(()=>{
        // TODO: test 0 accounts
        const t = accountStore.accounts.map(a => parseFloat(a.balance)).reduce((prev, current) => current + prev);
        return t.toString();
    });

</script>

<nav>
    <div class="px-2.5 flex flex-col">
        <a href="/">Transactions</a>
        <a href="/analytics">Spending</a>
    </div>
    <div class="h-[3px] w-full bg-neutral-100"></div>
    <ul>
        <li class="font-medium">
            <p>Accounts</p>
            <p>{formatMoney(total)}</p>
        </li>

        <!--TODO: test these and test total-->
        {#each accountStore.accounts as account (account.id)}
            <!--TODO: add title-->
            <!--FIXME: use accountBalance method-->
            <li>
                <p class="text-truncate max-w-[50%]">{account.name}</p>
                <p>{formatMoney(account.balance)}</p>
            </li>
        {/each}
        <Button variant="neutral" size="small">
            <!--TODO: change stoke width-->
            <Plus size="12"/>
            Add account
        </Button>
    </ul>
</nav>

<style>
	nav{
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
        gap: 24px;
        padding-block: 12px;
		background-color: var(--color-neutral-25);
        border-right: 1px solid var(--color-neutral-100);
	}

    ul{
        display: flex;
        flex-direction: column;
        gap: 12px;
        padding-inline: 20px;
    }

    li{
        display: flex;
        justify-content: space-between;
    }
</style>
