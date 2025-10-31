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
    $inspect(appStore.budgets);
</script>

<ul class="expense-table">
	<p class="table-heading">Category</p>
	<p class="table-heading">Total</p>
	<p class="table-heading">Spent</p>
	<p class="table-heading">Remaining</p>
	{#each appStore.budgets as budget (budget.category.id)}
		<Budget {budget}/>
	{/each}
</ul>

<style>
	.expense-table{
		display: grid;
		grid-template-columns: repeat(4,1fr);
	}

	.table-heading{
		color: var(--color-text-muted);
		padding: 12px;
	}
</style>
