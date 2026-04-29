
<script>
    import { Plus } from "@lucide/svelte";
    import { Button } from "$components/button";
    import {accountBalance, accountStore} from "$lib/stores/account.svelte";
    import { Popover, PopoverContent,PopoverTrigger } from "$components/popover";
    import TextField from "$components/TextField.svelte";
    import Account from "./Account.svelte";
    import { formatMoney } from "$lib/utils/money";
    import {transactionStore} from "$lib/stores/transaction.svelte.js";

    // TODO: test 0 accounts
    const total = $derived.by(() => {
        const transactions = transactionStore.transactions;
        const t = accountStore
            .accounts
            .map(a => accountBalance(a.id,transactions))
            .reduce((prev, current) => current + prev,0);
        return t.toString();
    });

    let name = $state("My account");
    let popoverOpen = $state(false);
	let startingBalance = $state("0.00");

	async function createAccount() {
	    popoverOpen = false;
	    await accountStore.createAccount({ name,startingBalance });
	}
</script>

<section class="account-section">
    <div class="font-medium flex justify-between">
        <h6 class="text-body">Accounts</h6>
        <p>{formatMoney(total)}</p>
    </div>
    <ul class="account-list">
        {#each accountStore.accounts as account (account.id)}
            <Account {account}/>
        {/each}
    </ul>
    <Popover bind:open={popoverOpen}>
        <PopoverTrigger class="btn btn-small btn-neutral">
            <Plus/>
            Add account
        </PopoverTrigger>
        <PopoverContent class="space-y-1.5 p-2 w-(--bits-popover-anchor-width)">
            <TextField bind:value={name} label="Name"/>
            <TextField bind:value={startingBalance} label="Starting balance"/>
            <Button class="w-full" onclick={createAccount}>Save changes</Button>
        </PopoverContent>
    </Popover>
</section>

<style>
    .account-section{
        display: flex;
        flex-direction: column;
        gap: 16px;
        padding-inline: 20px;

        :global([data-expanded="false"]) &{
            display: none;
        }
    }

    .account-list{
        display: flex;
        flex-direction: column;
        gap: 12px;
        max-height: 350px;
        overflow-y: auto;
    }

    li{
        display: flex;
        justify-content: space-between;
    }
</style>
