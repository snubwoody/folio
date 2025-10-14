<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import AccountSection from "../components/home/AccountSection.svelte";
    import TransactionsSection from "../components/home/TransactionsSection.svelte";
</script>

<main>
	<AccountSection/>
	<TransactionsSection/>
</main>

<style>
	main{
		display: flex;
		flex-direction: column;
		gap: 32px;
		padding: 40px;
	}
</style>
