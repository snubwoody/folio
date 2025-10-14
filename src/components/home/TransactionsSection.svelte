<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
    import TransactionTable from "./TransactionTable.svelte";
	async function createExpense() {
		invoke("create_expense");
	}
</script>

<section>
	<header class="flex items-center justify-between">
		<p>
			Expenses
		</p>
		<button class="btn btn-primary" onclick={createExpense}>New</button>
	</header>
	<TransactionTable/>
</section>