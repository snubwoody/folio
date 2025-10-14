<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
    import TransactionTable from "./TransactionTable.svelte";
    import { transactionStore, type CreateExpense } from "../../lib/transaction.svelte";
	import {Popover} from "melt/builders";
    import TextField from "../TextField.svelte";


	let amount = $state("");
	async function createExpense() {
		const date = Date.now();
		console.log(date);
		transactionStore.addExpense({date: "2024-10-10",amount:"0",currencyCode:"USD"})
	}
	const popover = new Popover();
</script>


<header class="flex items-center justify-between">
	<p>Expenses</p>
	<button {...popover.trigger} class="btn btn-primary">New</button>
	<form {...popover.content} class="popoup-overlay" onsubmit={()=>{}}>
		<TextField bind:value={amount} label="Amount"/>
		<TextField bind:value={amount} label="Amount"/>
		<button class="btn btn-primary w-full">Add transaction</button>
	</form>
</header>