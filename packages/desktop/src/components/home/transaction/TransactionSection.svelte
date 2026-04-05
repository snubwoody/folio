<script lang="ts">
    import TransactionTable from "./TransactionTable.svelte";
    import Toolbar from "./Toolbar.svelte";
    import Actionbar from "./Actionbar.svelte";
    import { TableStore } from "$lib/stores/table.svelte";
    import {Table, TableCell, TableHeader, TableRow} from "$components/table";
    const tableStore = new TableStore();
    import {transactionStore} from "$lib/stores/transaction.svelte";
    import Transaction from "$components/home/transaction/Transaction.svelte";
</script>

<section>
    <Toolbar/>
    <TransactionTable {tableStore}/>
    <Actionbar {tableStore}/>
</section>
