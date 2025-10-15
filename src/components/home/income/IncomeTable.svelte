<script lang="ts">
    import { onMount } from "svelte";
    import { transactionStore } from "../../../lib/transaction.svelte";
    import Income from "./Income.svelte";

	onMount(async()=>{
	    await transactionStore.load();
	});
</script>

<ul class="expense-table">
	<p class="table-heading">Income stream</p>
	<p class="table-heading">Account</p>
	<p class="table-heading">Date</p>
	<p class="table-heading">Amount</p>
	{#each transactionStore.incomes as income (income.id)}
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
