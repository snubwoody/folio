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
    import { TableCell } from "$components/table";
    import { Select } from "bits-ui";

    interface Props{
        items: {value: string, label: string}[],
        // TODO: make bindable?
        value?: string,
        /**
         * Callback that runs when the selected value changes.
         * @param value
         */
        onChange?: (value: string) => void,
    }

    const { items,value,onChange }: Props = $props();

    let item = items.find(item => item.value === value) ?? items[0];
    let selectedItem = $state(item);
    // TODO: add selected style

    const onValueChange = (value: string) => {
        selectedItem = items.find(item => item.value === value) ?? selectedItem;
        onChange?.(value);
    };
</script>

<TableCell>
    <Select.Root {onValueChange} type="single" name="Combobox">
        <Select.Trigger class="w-full h-full flex justify-start outline-none">
            {selectedItem?.label ?? ""}
        </Select.Trigger>
        <Select.Portal>
            <Select.Content class="w-[var(--bits-select-anchor-width)] popup-overlay space-y-1">
                {#each items as item (item.value)}
                    <Select.Item value={item.value} label={item.label} class="select-item">
                        {#snippet children()}
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

    :global(.select-item) {
        border-radius: var(--radius-sm);
        padding: 8px;
        transition: all 250ms;

        &:hover {
            background-color: var(--color-neutral-50);
        }

        &[data-selected="true"] {
            background-color: var(--color-neutral-50);
        }
    }
</style>