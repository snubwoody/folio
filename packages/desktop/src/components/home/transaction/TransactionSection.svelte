<script lang="ts">
    import TransactionTable from "./TransactionTable.svelte";
    import Toolbar from "./Toolbar.svelte";
    import Actionbar from "./Actionbar.svelte";
    import { TableStore } from "$lib/stores/table.svelte";
    import {Table, TableCell, TableHeader, TableRow} from "$components/table";
    const tableStore = new TableStore();
    import {transactionStore} from "$lib/stores/transaction.svelte";
    import Transaction from "$components/home/transaction/Transaction.svelte";
    let selected = $derived(tableStore.allRowsSelected);
</script>

<section>
    <Toolbar/>
    <Table {tableStore}>
        <TableHeader>
            <TableCell class="w-fit shrink">
                <input type="checkbox" checked={selected} name="selected" id="row-checkbox"
                       onclick={(e) => {
                    if(e.currentTarget.checked){
                        tableStore.toggleSelectAll();
                        return;
                    }
                    tableStore.toggleSelectAll();
                }}
                >
            </TableCell>
            <TableCell>Date</TableCell>
            <TableCell>Account</TableCell>
            <TableCell>Payee</TableCell>
            <TableCell>Category</TableCell>
            <TableCell>Outflow</TableCell>
            <TableCell>Inflow</TableCell>
        </TableHeader>
        {#each transactionStore.transactions as transaction (transaction.id)}
            <Transaction {transaction} {tableStore}/>
        {/each}
    </Table>
    <TransactionTable {tableStore}/>
    <Actionbar {tableStore}/>
</section>
