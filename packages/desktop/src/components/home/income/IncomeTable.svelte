<script lang="ts">
    import Income from "./Income.svelte";
    import { appStore } from "$lib/state.svelte";
</script>

<ul class="expense-table">
	<p class="table-heading">Income stream</p>
	<p class="table-heading">Account</p>
	<p class="table-heading">Date</p>
	<p class="table-heading">Amount</p>
	{#each appStore.incomes as income (income.id)}
		<Income {income}/>
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
