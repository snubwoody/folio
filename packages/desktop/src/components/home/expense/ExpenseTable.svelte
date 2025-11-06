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
