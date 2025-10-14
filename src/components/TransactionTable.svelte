<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

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

	<p class="table-cell">Takeout</p>
	<p class="table-cell">Savings</p>
	<p class="table-cell">April 25, 2025</p>
	<p class="table-cell">$ 20.24</p>

	<p class="table-cell">Takeout</p>
	<p class="table-cell">Savings</p>
	<p class="table-cell">April 25, 2025</p>
	<p class="table-cell">$ 20.24</p>

	<p class="table-cell">Takeout</p>
	<p class="table-cell">Savings</p>
	<p class="table-cell">April 25, 2025</p>
	<p class="table-cell">$ 20.24</p>

	<p class="table-cell">Takeout</p>
	<p class="table-cell">Savings</p>
	<p class="table-cell">April 25, 2025</p>
	<p class="table-cell">$ 20.24</p>
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

	.table-cell{
		padding: 12px;
		border-right: 1px solid var(--color-neutral-50);
		border-bottom: 1px solid var(--color-neutral-50);
		border-top: 1px solid var(--color-neutral-50);
	}
</style>