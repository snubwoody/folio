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
    import type { HTMLAttributes } from "svelte/elements";
    import { TableStore } from "$lib/stores/table.svelte";
    import {transactionStore} from "$lib/stores/transaction.svelte";
    import DateCell from "./DateCell.svelte";
    import AccountCell from "./AccountCell.svelte";
    import {accountStore} from "$lib/stores/account.svelte";
    import SelectCell from "$components/home/transaction/SelectCell.svelte";
    import type {Snippet} from "svelte";

    interface Props extends  HTMLAttributes<HTMLDivElement>{
        tableStore: TableStore
        children?: Snippet
    }


    const { tableStore,children }:Props = $props();
    // TODO: add checkbox
    // TODO: background color when focused
    // TODO: background color on hover?
    // TODO: move account cell after completing
</script>

<div>
    <div class="table-header">
        <div class="t-cell">Date</div>
        <div class="t-cell">Account</div>
        <div class="t-cell">Payee</div>
        <div class="t-cell">Note</div>
        <div class="t-cell">Category</div>
        <div class="t-cell">Outflow</div>
        <div class="t-cell">Inflow</div>
    </div>
    {@render children?.()}
</div>

<style>
    .table-row{
        display: flex;
        border-bottom: 1px solid var(--color-neutral-50);
    }

    .table-header{
        display: flex;
        border-bottom: 1px solid var(--color-neutral-50);
    }

    /**TODO: Rename to table-cell **/
    :global(.t-cell){
        padding: 8px 16px;
        flex: 1;
    }
</style>