<!--@component
Displays a list of options to pick from.
--->
<script lang="ts">
    import { Select } from "bits-ui";

    interface Props{
        items: {value: string, label: string}[],
        value?: string,
        /**
         * Callback that runs when the selected value changes.
         * @param value
         */
        onChange?: (value: string) => void,
    }

    const { items,value,onChange }: Props = $props();

    let selectedItem = $derived(items.find(item => item.value === value));
    // TODO: add style for selected items

    const onValueChange = (value: string) => {
        selectedItem = items.find(item => item.value === value) ?? selectedItem;
        onChange?.(value);
    };
</script>

<Select.Root {onValueChange} type="single" name="Combobox">
    <Select.Trigger class="w-full h-full flex justify-start outline-none">
        {#if selectedItem}
            <p>{selectedItem.label}</p>
        {:else}
            <p class="invisible">Select a item</p>
        {/if}
    </Select.Trigger>
    <Select.Portal>
        <!-- FIXME: make it fit the children -->
        <Select.Content class="w-(--bits-select-anchor-width) popup-overlay space-y-1">
            {#each items as item (item.value)}
                <Select.Item value={item.value} label={item.label} class="select-item">
                    {item.label}
                </Select.Item>
            {/each}
        </Select.Content>
    </Select.Portal>
</Select.Root>

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
