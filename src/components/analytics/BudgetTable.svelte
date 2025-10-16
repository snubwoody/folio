<script lang="ts">
    import { appStore } from "$lib/state.svelte";
    import Budget from "./Budget.svelte";
</script>

<ul class="expense-table">
	<p class="table-heading">Income stream</p>
	<p class="table-heading">Total</p>
	{#each appStore.budgets as budget (budget.category.id)}
		<Budget {budget}/>
	{/each}
</ul>

<style>
	.expense-table{
		display: grid;
		grid-template-columns: repeat(2,1fr);
	}

	.table-heading{
		color: var(--color-text-muted);
		padding: 12px;
	}
</style>
