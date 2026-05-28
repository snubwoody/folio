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
    import { Dialog } from "bits-ui";
    import { open } from "@tauri-apps/plugin-dialog";
    import { invoke } from "@tauri-apps/api/core";
    import { CirclePlus, ChevronsUpDown } from "@lucide/svelte";
    import { IconButton, TextButton } from "$components/button";
    import { Table, TableCell, TableHeader, TableRow } from "$components/table";

    let rows =  $state([]);

    // TODO:
    // - Add dropdowns in the column headers
    // - Allow discarding rows and columns

    async function importCsv(){
        const file = await open({
            multiple: false,
            directory: false,
            filters: [{name:"csv-filter",extensions:["csv"]}]
        });
        rows = await invoke("load_csv", {path: file});
        console.log(rows);
    }
    // TODO: handle headers
    // TODO: add global dialog-portal class
    // TODO: add negative/inverse
</script>

<Dialog.Root>
    <Dialog.Trigger>
        <TextButton theme="primary" onclick={importCsv}>
            <CirclePlus />
            Import
        </TextButton>
    </Dialog.Trigger>
    <Dialog.Portal>
        <Dialog.Content class="settings-portal import-csv-portal">
            <header>
                <h4>Import CSV</h4>
                <p>Path: </p>
            </header>
            <Table length={5}>
                <TableHeader>
                    <TableCell class="flex justify-between items-center">
                        Date
                        <IconButton variant="ghost">
                            <ChevronsUpDown />
                        </IconButton>
                    </TableCell>
                    <TableCell>Category</TableCell>
                    <TableCell>Note</TableCell>
                    <TableCell>Outflow</TableCell>
                    <TableCell>Inflow</TableCell>
                </TableHeader>
                {#each rows as row}
                    <TableRow>
                        {#each row as cell}
                            <TableCell>{cell}</TableCell>
                        {/each}
                    </TableRow>
                {/each}
            </Table>
        </Dialog.Content>
    </Dialog.Portal>
</Dialog.Root>

<style>
    @keyframes fade-in{
        from{
            opacity: 0;
        }
    }

    :global(.import-csv-portal){
        padding: 32px;
    }

    :global(.settings-portal){
        position: fixed;
        inset: 6rem 12rem;
        background: var(--color-white);
        z-index: 200;
        border-radius: var(--radius-lg);
        box-shadow: var(--shadow-md);
        overflow: hidden;
    }
</style>
