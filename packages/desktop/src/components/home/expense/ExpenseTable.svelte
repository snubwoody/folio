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
    import Expense from "./Expense.svelte";
    import { Table, TableHeader } from "$components/table";
    import { appStore } from "$lib/state.svelte";
    import type {DataCellParams, DataColumn} from "$lib/table";

    const columns: DataColumn[] = [
        {id: "Category"},
        {id: "Account"},
        {id: "Date"},
        {id: "Amount"},
    ];

    const cells: DataCellParams[] = [];

    $effect(()=>{
        appStore.expenses.forEach(expense => {
            // FIXME allow null values
            cells.push({value: expense.category?.title ?? ""});
            cells.push({value: expense.account?.name ?? ""});
            cells.push({value: expense.date});
            cells.push({value: expense.amount});
        })
    })
</script>

<Table {cells} {columns}>
    {#snippet header(label)}
        <TableHeader>{label}</TableHeader>
    {/snippet}
	{#each appStore.expenses as expense (expense.id)}
		<Expense {expense}/>
	{/each}
</Table>

