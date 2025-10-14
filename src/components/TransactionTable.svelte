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

<section>
	<p class="table-heading">Category</p>
	<p class="table-heading">Account</p>
	<p class="table-heading">Date</p>
	<p class="table-heading">Amount</p>

	<p>Category</p>
	<p>Account</p>
	<p>Date</p>
	<p>Amount</p>
</section>

<style>
	section{
		display: grid;
		grid-template-columns: repeat(4,1fr);
	}	

	.table-heading{
		color: var(--color-text-muted);
	}
</style>