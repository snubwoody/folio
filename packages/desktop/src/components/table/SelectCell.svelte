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
<script>
    import { TableCell } from "$components/table";
    import { Select } from "bits-ui";

    const items = [
        { value: "RBC Credit Card",label: "RBC Credit Card" },
        { value: "RBC Savings",label: "RBC Savings" },
        { value: "Absa",label: "Absa" },
        { value: "FNB",label: "FNB" }
    ];
</script>

<TableCell>
    <Select.Root type="single" name="Combobox">
        <Select.Trigger class="w-full h-full flex justify-start outline-none">Trigger</Select.Trigger>
        <Select.Portal>
            <Select.Content class="w-[var(--bits-select-anchor-width)]">
                {#each items as item,index}
                    <Select.Item value={item.value} label={item.label}>
                        {#snippet children({ selected })}
                            {item.label}
                        {/snippet}
                    </Select.Item>
                {/each}
            </Select.Content>
        </Select.Portal>
    </Select.Root>
</TableCell>

<style>
    :global([data-select-content]){
        background: white;
    }
</style>