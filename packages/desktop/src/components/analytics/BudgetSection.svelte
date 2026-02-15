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
    import { appStore } from "$lib/state.svelte";
    import Budget from "./Budget.svelte";
    import CreateBudget from "./CreateBudget.svelte";
</script>

<section>
    <ul class="budget-table">
        <div class="flex items-center gap-1.5">
            <h6>Categories</h6>
            <CreateBudget/>
        </div>
        <h6>Total</h6>
        <h6>Left to spend</h6>
        <!-- <p class="table-heading">Remaining</p> -->
        {#each appStore.budgets as budget (budget.category.id)}
            <Budget {budget}/>
            <div class="bg-neutral-50 w-full h-[1px] col-span-3"></div>
        {/each}
    </ul>
</section>

<style>
	.budget-table{
		display: grid;
		grid-template-columns: 1fr auto auto;
        align-items: center;
        column-gap: 56px;
        row-gap: 16px;
	}
</style>
