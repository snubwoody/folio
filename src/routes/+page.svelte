<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
    import TransactionTable from "../components/TransactionTable.svelte";

	async function createExpense() {
		invoke("create_expense");
	}
</script>

<main>
	<section>
		<header>
			<p>
				Expenses
			</p>
			<button class="btn btn-primary" onclick={createExpense}>New</button>
		</header>
		<TransactionTable/>
	</section>
</main>

<style>
</style>
