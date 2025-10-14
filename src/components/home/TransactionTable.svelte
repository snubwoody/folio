<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { transactionStore } from "../../lib/transaction.svelte";
    import Expense from "./Expense.svelte";

	onMount(async()=>{
	    await transactionStore.load();
	});
</script>

<ul class="expense-table">
	<p class="table-heading">Category</p>
	<p class="table-heading">Account</p>
	<p class="table-heading">Date</p>
	<p class="table-heading">Amount</p>
	{#each transactionStore.expenses as expense (expense.id)}
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
