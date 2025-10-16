<script lang="ts">
	import { Popover } from "melt/builders";
    import TextField from "../TextField.svelte";
    import DateField from "../DateField.svelte";
    import SelectMenu from "$components/SelectMenu.svelte";
    import type { Account, Category } from "$lib/lib";
    import { appStore } from "$lib/state.svelte";

	const popover = new Popover();

	let amount = $state("");
    // TODO: move menu button to a new component
	let account: Account | undefined;
	let category: Category | undefined;
    let date: string | undefined = $state(undefined);
	async function createExpense() {
	    appStore.transactions.addExpense({
	        date: date,
	        amount:amount,
	        categoryId: category?.id,
	        currencyCode:"USD",
	        accountId: account?.id,
	    });
	    popover.open = false;

	}
</script>

<button {...popover.trigger} class="btn btn-primary ml-auto">New</button>
<form {...popover.content} class="popup-overlay space-y-2 bg-white max-w-[350px] w-full" onsubmit={()=>{}}>
    <TextField bind:value={amount} label="Amount"/>
    <DateField onChange={(year,month,day) => date = `${year}-${month}-${day}`}/>
    <SelectMenu
        label="Account"
        items={appStore.accounts}
        toOption={(a) => {return { label: a.name,value: a.id };}}
        onChange={(item) => account = item}
    />
    <SelectMenu
        label="Category"
        items={appStore.categories}
        toOption={(a) => {return { label: a.title,value: a.id };}}
        onChange={(item) => category = item}
    />
    <button class="btn btn-primary w-full" onclick={createExpense}>Add transaction</button>
</form>

