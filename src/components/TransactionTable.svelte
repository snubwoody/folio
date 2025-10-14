<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
	type Expense = {
		id: string,
	}

	onMount(async()=>{
		let expenses = await invoke("fetch_expenses");
		console.log(expenses);
	});
</script>

<ul class="expense-table">
	<p class="table-heading">Category</p>
	<p class="table-heading">Account</p>
	<p class="table-heading">Date</p>
	<p class="table-heading">Amount</p>

	<p class="table-cell">Category</p>
	<p class="table-cell">Account</p>
	<p class="table-cell">Date</p>
	<p class="table-cell">Amount</p>
</ul>

<style>
	.expense-table{
		display: grid;
		grid-template-columns: repeat(4,1fr);
	}	

	.table-heading{
		color: var(--color-text-muted);
	}

	.table-cell{
		border-left: 1px solid var(--color-neutral-50);
		border-bottom: 1px solid var(--color-neutral-50);
	}
</style>