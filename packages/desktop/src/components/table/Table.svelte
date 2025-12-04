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
    import {type DataCellParams, type DataColumn, DataTable} from "$lib/table";

    interface Props{
        header: Snippet<[string]>,
        children: Snippet,
        columns: DataColumn[],
        cells: DataCellParams[],
    }

    const { children,header,columns,cells }: Props = $props();
    const table = new DataTable(columns,cells);
</script>


<div>
    {#each columns as column (column.id)}
        {@render header(column.id)}
    {/each}
    {@render children()}
</div>

<style>
    div{
        display: grid;
        grid-template-columns: repeat(4,1fr);
    }
</style>