<script lang="ts">
    import { TableStore } from "$lib/stores/table.svelte";
    import Actionbar from "./Actionbar.svelte";
    import Toolbar from "./Toolbar.svelte";
    import TransactionTable from "./TransactionTable.svelte";

    type Props = {
        accountId?: string;
    };

    const tableStore = new TableStore();

    const { accountId }: Props = $props();
</script>

<section>
    <Toolbar {accountId}/>
    <TransactionTable {accountId} {tableStore}/>
    <Actionbar {tableStore}/>
</section>
