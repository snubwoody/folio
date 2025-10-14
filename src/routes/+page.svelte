<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
    import TransactionTable from "../components/TransactionTable.svelte";
	import AccountSection from "../components/home/AccountSection.svelte";
	async function createExpense() {
		invoke("create_expense");
	}
</script>

<main>
	<AccountSection/>
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
	main{
		display: flex;
		flex-direction: column;
		gap: 32px;
		padding: 40px;
	}
</style>
