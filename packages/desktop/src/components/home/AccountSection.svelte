<script lang="ts">
    import TextField from "../TextField.svelte";
    import { accountStore } from "$lib/stores/account.svelte";
    import { Button } from "$components/button";
    import { Plus } from "@lucide/svelte";
    import Account from "./Account.svelte";
    import { Popover, PopoverContent,PopoverTrigger } from "$components/popover";

	let name = $state("My account");
    let popoverOpen = $state(false);
	let startingBalance = $state("0.00");

	async function createAccount() {
	    popoverOpen = false;
	    await accountStore.createAccount({ name,startingBalance });
	}
</script>

<section>
	<header class="flex items-center justify-between">
		<h6>Accounts</h6>
        <Popover bind:open={popoverOpen}>
            <PopoverTrigger class="icon-btn icon-btn-medium icon-btn-ghost">
                <Plus/>
            </PopoverTrigger>
            <PopoverContent class="space-y-1.5 p-2">
                <TextField bind:value={name} label="Name"/>
                <TextField bind:value={startingBalance} label="Starting balance"/>
                <Button class="w-full" onclick={createAccount}>Save changes</Button>
            </PopoverContent>
        </Popover>
	</header>
	<ul>
		{#each accountStore.accounts as account (account.id)}
            <Account {account}/>
		{/each}
	</ul>
</section>

<style>
	section{
		display: grid;
		gap: 20px;
        padding: 24px;
	}

	ul{
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px,1fr));
		gap: 20px;
	}
</style>
