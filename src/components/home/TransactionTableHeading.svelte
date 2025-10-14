<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
    import TransactionTable from "./TransactionTable.svelte";
    import { transactionStore, type CreateExpense } from "../../lib/transaction.svelte";
	import {Popover} from "melt/builders";
    import TextField from "../TextField.svelte";
    import DateField from "../DateField.svelte";
    import SelectMenu from "../SelectMenu.svelte";
    import { accountStore } from "../../lib/account.svelte";


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
	<form {...popover.content} class="popup-overlay space-y-2 bg-white max-w-[350px] w-full" onsubmit={()=>{}}>
		<TextField bind:value={amount} label="Amount"/>
		<DateField/>
		<SelectMenu 
			label="Account"
			items={accountStore.accounts} 
			defaultValue={accountStore.accounts[0]}
			toOption={(a) => {return {label: a.name,value: a.id}}}
			onChange={(item) => console.log("Updated",item)}
		/>
		<SelectMenu 
			label="Category"
			items={accountStore.accounts} 
			defaultValue={accountStore.accounts[0]}
			toOption={(a) => {return {label: a.name,value: a.id}}}
			onChange={(item) => console.log("Updated",item)}
		/>
		<button class="btn btn-primary w-full">Add transaction</button>
	</form>
</header>