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
    import Expense from "./Expense.svelte";
    import { appStore } from "$lib/state.svelte";
</script>

<ul class="expense-table">
	<li class="table-heading">Category</li>
	<li class="table-heading">Account</li>
	<li class="table-heading">Date</li>
	<li class="table-heading">Amount</li>
	{#each appStore.expenses as expense (expense.id)}
		<Expense {expense}/>
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
