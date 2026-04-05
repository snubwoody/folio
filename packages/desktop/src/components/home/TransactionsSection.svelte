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
    import {Table, TableRow} from "$components/table";
    const tableStore = new TableStore();
    import {transactionStore} from "$lib/stores/transaction.svelte";
    import DateCell from "$components/table/DateCell.svelte";
    import AccountCell from "$components/table/AccountCell.svelte";
</script>

<section>
    <Toolbar/>
    <Table {tableStore}>
        {#each transactionStore.transactions as transaction (transaction.id)}
            <TableRow>
                <DateCell {transaction}/>
                <AccountCell {transaction}/>
                <div class="t-cell">Payee</div>
                <div class="t-cell">Note</div>
                <div class="t-cell">Category</div>
                <div class="t-cell">Outflow</div>
                <div class="t-cell">Inflow</div>
            </TableRow>
        {/each}
    </Table>
    <TransactionTable {tableStore}/>
    <Actionbar {tableStore}/>
</section>
