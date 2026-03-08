<script lang="ts">
	import { Popover } from "melt/builders";
    import TextField from "../TextField.svelte";
    import { accountStore } from "$lib/account.svelte";
    import { IconButton,Button } from "$components/button";
    import { Plus } from "@lucide/svelte";
    import Account from "./Account.svelte";

	const popover = new Popover();
	let name = $state("My account");
	let startingBalance = $state("0.00");

	async function createAccount() {
	    await accountStore.createAccount({ name,startingBalance });
	    popover.open = false;
	}
    // TODO: replace popover with bits-ui
</script>

<section>
	<header class="flex items-center justify-between">
		<h6>Accounts</h6>
        <IconButton {...popover.trigger} variant="ghost">
            <Plus/>
        </IconButton>
		<form class="popup-overlay space-y-1.5" {...popover.content}>
			<TextField bind:value={name} label="Name"/>
			<TextField bind:value={startingBalance} label="Starting balance"/>
            <Button class="w-full" onclick={createAccount}>Save changes</Button>
		</form>
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
