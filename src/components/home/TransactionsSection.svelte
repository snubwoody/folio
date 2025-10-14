<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
    import TransactionTable from "./TransactionTable.svelte";
    import TransactionTableHeading from "./TransactionTableHeading.svelte";
</script>

<section>
	<TransactionTableHeading/>
	<TransactionTable/>
</section>