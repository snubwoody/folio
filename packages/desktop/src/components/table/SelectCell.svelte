<!--@component
Displays a list of options to pick from.
--->
<script lang="ts">
    import { Select } from "bits-ui";
    import {TableCell} from "$components/table";
    import type { HTMLAttributes } from "svelte/elements";

    interface Props extends HTMLAttributes<HTMLDivElement>{
        items: {value: string, label: string}[],
        value?: string,
        /**
         * Callback that runs when the selected value changes.
         * @param value
         */
        onChange?: (value: string) => void
    }

    const { items,value,onChange,...rest }: Props = $props();

    let selectedItem = $derived(items.find(item => item.value === value));
    

    const onValueChange = (value: string) => {
        selectedItem = items.find(item => item.value === value) ?? selectedItem;
        onChange?.(value);
    };
</script>

<TableCell {...rest}>
    <Select.Root {onValueChange} type="single" name="Combobox">
        <Select.Trigger>
            {#if selectedItem}
                <p>{selectedItem.label}</p>
            {:else}
                <p class="invisible">Select a item</p>
            {/if}
        </Select.Trigger>
        <Select.Portal>
            <!-- FIXME: make it fit the children -->
            <Select.Content class="popup-overlay space-y-1 select-content">
                {#each items as item (item.value)}
                    <Select.Item value={item.value} label={item.label} class="select-item">
                        {item.label}
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

    :global(.select-content){
        width: var(--bits-select-anchor-width);
        min-width: var(--bits-select-anchor-width);
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
