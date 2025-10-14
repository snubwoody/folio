<script lang="ts">
	import {Popover} from "melt/builders";
    import TextField from "../TextField.svelte";
    import { scale } from "svelte/transition";

	const popover = new Popover();
	let name = $state("My account");
	let startingBalance = $state("0.00");

	async function createAccount() {
		popover.open = false;
	}
</script>

<section>
	<header class="flex items-center justify-between">
		<h6>Accounts</h6>

		<button {...popover.trigger} class="icon-btn icon-btn-grey icon-btn-small">
			<i class="ph ph-plus"></i>
		</button>
		<form class="shadow-md p-2 rounded-md space-y-1.5" {...popover.content}>
			<TextField bind:value={name} label="Name"/>
			<TextField bind:value={startingBalance} label="Starting balance"/>
			<!--TODO: add text button-->
			<button class="btn btn-primary w-full" onclick={createAccount}>Save changes</button>
		</form>
	</header>
	<ul>
		<li class="shadow-xl">
			<p>Savings</p>
			<h6>$ 53.35</h6>
		</li>
		<li>
			<p>Savings</p>
			<h6>$ 53.35</h6>
		</li>
	</ul>
</section>

<style>
	@keyframes animate-popup{
		from {
			opacity: 0.8;
			scale: 0.9;
		}
	}

	section{
		display: grid;
		gap: 20px
	}

	ul{
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px,1fr));
		gap: 20px;
	}

	li{
	}

	[data-melt-popover-content]{
		transition: all 250ms;
		opacity: 0.5;
		transform: scale(0);
	}
	
	[data-melt-popover-content][data-open]{
		opacity: 1;
		transform: scale(1);
		animation: animate-popup ease-in-out 100ms;
	}
</style>