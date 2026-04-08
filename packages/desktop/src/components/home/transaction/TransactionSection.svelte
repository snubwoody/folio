<script lang="ts">
    import TransactionTable from "./TransactionTable.svelte";
    import Toolbar from "./Toolbar.svelte";
    import Actionbar from "./Actionbar.svelte";
    import { TableStore } from "$lib/stores/table.svelte";
    const tableStore = new TableStore();
</script>

<section>
    <Toolbar/>
    <TransactionTable {tableStore}/>
    <Actionbar {tableStore}/>
</section>
