<script lang="ts">
	import { Popover } from "melt/builders";
    import TextField from "../TextField.svelte";
    import DateField from "../DateField.svelte";
    import SelectMenu from "$components/SelectMenu.svelte";
    import type { Account, Category, IncomeStream } from "$lib/lib";
    import { appStore } from "$lib/state.svelte";

	const popover = new Popover();

	let amount = $state("");
	let category: Category | undefined;
	async function createBudget() {
        if (!category) return;
	    appStore.createBudget(amount,category?.id);
	    popover.open = false;

	}
    // TODO: filter categories that don't have budgets
</script>

<button {...popover.trigger} class="icon-btn icon-btn-medium icon-btn-grey">
    <i class="ph ph-plus"></i>
</button>
<form {...popover.content} class="popup-overlay space-y-2 bg-white max-w-[350px] w-full" onsubmit={()=>{}}>
    <TextField bind:value={amount} label="Amount"/>
    <SelectMenu
        label="Categories"
        items={appStore.categories}
        toOption={(a) => {return { label: a.title,value: a.id };}}
        onChange={(item) => category = item}
    />
    <button class="btn btn-primary w-full" onclick={createBudget}>Add transaction</button>
</form>

