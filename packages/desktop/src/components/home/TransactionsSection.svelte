<!--
Copyright (C) 2025 Wakunguma Kalimukwa

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
-->
<script lang="ts">
    import TransactionTable from "./TransactionTable.svelte";
    import Toolbar from "./transaction/Toolbar.svelte";
    import Actionbar from "./transaction/Actionbar.svelte";
    import { TableStore } from "$lib/stores/table.svelte";
    import {Table, TableCell, TableHeader, TableRow} from "$components/table";
    const tableStore = new TableStore();
    import {transactionStore} from "$lib/stores/transaction.svelte";
    import TransactionNew from "$components/home/transaction/TransactionNew.svelte";
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
            <TransactionNew {transaction} {tableStore}/>
        {/each}
    </Table>
    <TransactionTable {tableStore}/>
    <Actionbar {tableStore}/>
</section>
