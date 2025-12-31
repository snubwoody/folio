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
    import type { Snippet } from "svelte";
    import { type DataCell, type DataCellParams, type DataColumn, type DataRow, DataTable } from "$lib/table";
    import type { HTMLAttributes } from "svelte/elements";

    interface Props extends  HTMLAttributes<HTMLDivElement>{
        header: Snippet<[string]>,
        cell: Snippet<[DataCell]>,
        columns: DataColumn[],
        rows: DataRow[],
        cells: DataCellParams[],
    }

    const { header,cell,columns,rows,cells,...rest }: Props = $props();
    const table = new DataTable(columns,rows,cells);
</script>

<table class="transaction-table" {...rest}>
    <thead>
        <tr>
            {#each columns as column (column.id)}
                {@render header(column.id)}
            {/each}
        </tr>
    </thead>
    <tbody>
        {#each table.rows as row (row.id)}
            <tr>
                {#each table.rowCells(row.id) as dataCell,index (index)}
                    {@render cell(dataCell)}
                {/each}
            </tr>
        {/each}
    </tbody>
</table>

