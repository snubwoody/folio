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
    import {SelectCell, Table, TableCell, TableHeader} from "$components/table";
    import { appStore } from "$lib/state.svelte";
    import type {DataCell, DataCellParams, DataColumn, DataRow} from "$lib/table";
    import {type Account, type Category, formatDate} from "$lib/lib";
    import DatePicker from "$components/DatePicker.svelte";

    // TODO: maybe generic
    const columns: DataColumn[] = [
        {id: "Category"},
        {id: "Account"},
        {id: "Date"},
        {id: "Amount"},
    ];

    const rows: DataRow[] = $derived.by(()=>
        appStore.expenses.map(expense => {
            {return {id: expense.id}}
        })
    )

    const cells = $derived.by(()=>{
        const cells: DataCellParams[] = [];
        appStore.expenses.forEach(expense => {
            // FIXME allow null values
            cells.push({value: expense.category?.id ?? ""});
            cells.push({value: expense.account?.id ?? ""});
            cells.push({value: expense.date});
            cells.push({value: expense.amount});
        });
        return cells;
    })

    const categories = $derived.by(()=>
        appStore.categories.map(category => {return {value: category.id,label: category.title}})
    )
    const accounts = $derived.by(()=>
        appStore.accounts.map(account => {return {value: account.id,label: account.name}})
    )

    async function editCategory(expenseId: string,categoryId: string) {
        await appStore.transactions.editExpense({
            id: expenseId,
            categoryId
        });
    }

    async function editAccount(expenseId: string, accountId: string) {
        await appStore.transactions.editExpense({
            id: expenseId,
            accountId
        });
    }

    function updateDate(expenseId: string,year: number, month: number, day: number) {
        appStore.transactions.editExpense({
            id: expenseId,
            date: `${year}-${month}-${day}`
        });
    }
</script>

<Table {cells} {columns} {rows}>
    {#snippet header(label)}
        <TableHeader>{label}</TableHeader>
    {/snippet}
    {#snippet cell({value,columnId,rowId})}
        {#if columnId === "Category"}
            <SelectCell
                {value}
                onChange={(value) => editCategory(rowId,value)}
                items={categories}
            />
        {:else if columnId === "Account"}
            <SelectCell
                {value}
                onChange={(value) => editAccount(rowId,value)}
                items={accounts}
            />
        {:else if columnId === "Date"}
            <TableCell class="justify-between">
                <p>{formatDate(value)}</p>
                <DatePicker onDateChange={(year,month,day)=>updateDate(rowId,year,month,day)} />
            </TableCell>
        {:else}
            <TableCell>{value}</TableCell>
        {/if}
    {/snippet}
	{#each appStore.expenses as expense (expense.id)}
		<Expense {expense}/>
	{/each}
</Table>

