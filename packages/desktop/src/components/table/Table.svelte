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

    interface Props{
        children: Snippet
    }

    interface DataColumn{
        id: string
    }

    interface DataRow{
        id: string
    }

    type DataValue = string | number;

    interface DataCell{
        value: DataValue
    }

    const { children }: Props = $props();
</script>

<div>
    {@render children()}
</div>

<style>
    div{
        display: grid;
        grid-template-columns: repeat(4,1fr);
    }
</style>