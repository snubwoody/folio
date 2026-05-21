<script lang="ts">
    import TransactionTable from "./TransactionTable.svelte";
    import Toolbar from "./Toolbar.svelte";
    import Actionbar from "./Actionbar.svelte";
    import { TableStore } from "$lib/stores/table.svelte";

    type Props = {
        accountId?: string
    };

    const tableStore = new TableStore();

    const { accountId }:Props = $props();
</script>

<section>
    <Toolbar {accountId}/>
    <TransactionTable {accountId} {tableStore}/>
    <Actionbar {tableStore}/>
</section>
