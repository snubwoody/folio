<script lang="ts">
    import { transactionStore } from "../../lib/transaction.svelte";
	import { Popover } from "melt/builders";
    import TextField from "../TextField.svelte";
    import DateField from "../DateField.svelte";
    import SelectMenu from "$components/SelectMenu.svelte";
    import { accountStore } from "../../lib/account.svelte";
    import type { Account } from "$lib/lib";

	const popover = new Popover();

	let amount = $state("");
	let account:Account | undefined;
    let date: string | undefined = $state(undefined);
	async function createExpense() {
	    transactionStore.addExpense({
	        date: date,
	        amount:amount,
	        currencyCode:"USD",
	        accountId: account?.id,
	    });
	    popover.open = false;
	}
</script>

<header class="flex items-center justify-between">
	<p>Expenses</p>
	<button {...popover.trigger} class="btn btn-primary">New</button>
	<form {...popover.content} class="popup-overlay space-y-2 bg-white max-w-[350px] w-full" onsubmit={()=>{}}>
		<TextField bind:value={amount} label="Amount"/>
		<DateField onChange={(year,month,day) => date = `${year}-${month}-${day}`}/>
		<SelectMenu
			label="Account"
			items={accountStore.accounts}
			defaultValue={accountStore.accounts[0]}
			toOption={(a) => {return { label: a.name,value: a.id };}}
			onChange={(item) => account = item}
		/>
		<SelectMenu
			label="Category"
			items={accountStore.accounts}
			defaultValue={accountStore.accounts[0]}
			toOption={(a) => {return { label: a.name,value: a.id };}}
			onChange={(item) => console.log("Updated",item)}
		/>
		<button class="btn btn-primary w-full" onclick={createExpense}>Add transaction</button>
	</form>
</header>
