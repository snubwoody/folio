<!--
Copyright (C) 2025 Wakunguma Kalimukwa

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
-->
<script lang="ts">
	import { Popover } from "melt/builders";
    import TextField from "../TextField.svelte";
    import SelectMenu from "$components/SelectMenu.svelte";
    import type { Category } from "$lib/lib";
    import { appStore } from "$lib/state.svelte";

	const popover = new Popover();

	let amount = $state("");
	let category: Category | undefined;
	async function createBudget() {
	    if (!category) return;
	    await appStore.createBudget(amount,category?.id);
	    popover.open = false;
	}
    // TODO: filter categories that don't have budgets
</script>

<button {...popover.trigger} class="icon-btn icon-btn-medium icon-btn-grey">
    <i class="ph ph-plus"></i>
</button>
<form {...popover.content} class="popup-overlay space-y-2 bg-white max-w-[350px] w-full" onsubmit={() => {}}>
    <TextField bind:value={amount} label="Amount"/>
    <SelectMenu
        label="Categories"
        items={appStore.categories}
        toOption={(a) => {
            return { label: a.title,value: a.id };
        }}
        onChange={(item) => category = item}
    />
    <button class="btn btn-primary w-full" onclick={createBudget}>Add transaction</button>
</form>

