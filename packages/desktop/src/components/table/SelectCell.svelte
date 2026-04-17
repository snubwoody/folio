<script lang="ts">
    import { Select } from "bits-ui";
    import { TableCell } from "$components/table";
    import type { HTMLAttributes } from "svelte/elements";

    type SelectItem = {
        value: string,
        label: string,
        disabled?:boolean
    };

    interface Props extends HTMLAttributes<HTMLDivElement>{
        items: SelectItem[],
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
    // TODO: add UI for selected and disabled select menu items
</script>

<TableCell {...rest}>
    <Select.Root {onValueChange} type="single" name="Combobox">
        <Select.Trigger>
            {#if selectedItem}
                <p>{selectedItem.label}</p>
            {:else}
                <p class="invisible">Select an item</p>
            {/if}
        </Select.Trigger>
        <Select.Portal>
            <Select.Content class="popup-overlay space-y-1 select-content">
                {#each items as { value,label,disabled } (value)}
                    <Select.Item value={value} label={label} {disabled} class="select-item">
                        {label}
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

        &[data-disabled=""] {
            color: var(--color-text-muted);
        }

        &[data-disabled=""]:hover {
            background-color: initial;
        }
    }
</style>
