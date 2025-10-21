<script lang="ts">
	import { Popover } from "melt/builders";
    import TextField from "../TextField.svelte";
    import { appStore } from "$lib/state.svelte";
    import { formatAmount } from "$lib/lib";

	const popover = new Popover();
	let name = $state("My account");
	let startingBalance = $state("0.00");

	async function createAccount() {
	    await appStore.accountStore.addAccount(name,startingBalance);
	    popover.open = false;
	}
</script>

<section>
	<header class="flex items-center justify-between">
		<h6>Accounts</h6>
		<button {...popover.trigger} class="icon-btn icon-btn-grey icon-btn-small">
			<i class="ph ph-plus"></i>
		</button>
		<form class="popup-overlay space-y-1.5" {...popover.content}>
			<TextField bind:value={name} label="Name"/>
			<TextField bind:value={startingBalance} label="Starting balance"/>
			<!--TODO: add text button-->
			<button class="btn btn-primary w-full" onclick={createAccount}>Save changes</button>
		</form>
	</header>
	<ul>
		{#each appStore.accounts as account (account.id)}
			<li class="shadow-purple-sm p-2 rounded-md">
				<p>{account.name}</p>
				<h6>{formatAmount(account.balance)}</h6>
			</li>
		{/each}
	</ul>
</section>

<style>
	section{
		display: grid;
		gap: 20px
	}

	ul{
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px,1fr));
		gap: 20px;
	}

</style>
