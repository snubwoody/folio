<!--
@component
A select menu that displays a popup with options for the user to choose
from.

# Example
```svelte
<script>
    const paymentMethods = [
        {id: 298024, title: "Apple Pay"},
        {id: 232353, title: "Klarna"},
        {id: 368547, title: "Bank"},
        {id: 869822, title: "Credit card"},
        {id: 724024, title: "Cash"},
        {id: 239142, title: "PayPal"},
    ]
</script>

<SelectMenu
    label = "Payment method"
    items={paymentMethods}
    toOption = {(item) => {value: item.id, label: item.title}}
    onChange = {(item) => console.log(item)}
/>
```
-->
<script lang="ts">
    import { Select } from "melt/builders";
    import type { SelectOption } from "$lib/select.svelte";
    import { ChevronDown, StickyNote } from "@lucide/svelte";
    import {Select as SelectMenu} from "bits-ui";

    interface SelectItem {
        value: string,
        label: string
    }
    // TODO: replace toOption and just pass in a list of items
    type Props = {
        label?: string;
        items: SelectItem[];
        defaultValue?: SelectItem;
        onChange?: (value: string) => void;
        class?: string
    };

    const {
        label,
        items,
        onChange,
        class: userClass,
        defaultValue
    }: Props = $props();

    let selectedOption: SelectOption | undefined = $derived(
        defaultValue ?? undefined
    );

    // const options = $derived(items.map((i) => toOption(i)));

    // const onValueChange = (value?: SelectOption) => {
    //     const item = items.find((i) => toOption(i).value === value?.value);
    //     if (!item) {
    //         return;
    //     }

    //     selectedOption = toOption(item);
    //     onChange?.(item);
    // };

    // const select = $derived(new Select<SelectOption>({
    //     value: defaultValue ? toOption(defaultValue) : undefined,
    //     onValueChange: onValueChange
    // }));
</script>

<!---TODO: rename-->
<SelectMenu.Root type="single" onValueChange={onChange}>
    <SelectMenu.Trigger class={`select-trigger ${userClass}`}>
        {#if label}
            <p class="text-sm text-text-muted">{label}</p>
        {/if}
        {selectedOption?.label ?? "Select an option"}
        <ChevronDown class="select-btn-icon" size="20"/>
    </SelectMenu.Trigger>
    <SelectMenu.Portal>
        <SelectMenu.Content class="w-(--bits-select-anchor-width) popup-overlay space-y-1">
            {#each items as item (item.value)}
                <SelectMenu.Item value={item.value} label={item.label} class="select-item">
                    {item.label}
                </SelectMenu.Item>
            {/each}
        </SelectMenu.Content>
    </SelectMenu.Portal>
</SelectMenu.Root>

<!-- <div class={`space-y-1 ${className} select-none`}>
    {#if label}
        <p class="text-sm text-text-muted">{label}</p>
    {/if}
    <button
        {...select.trigger}
        class="flex w-full items-center justify-between px-1.5 py-1 rounded-md border border-neutral-50"
    >
        {selectedOption?.label ?? "Select an option"}
        <ChevronDown class="select-btn-icon" size="20"/>
    </button>
</div> -->

<!-- <ul {...select.content} class="popup-overlay space-y-1">
    {#each options as option (option.value)}
        {@const selected = select.isSelected(option)}
        <li {...select.getOption(option)} data-selected={selected}>
            {option.label}
        </li>
    {/each}
</ul> -->

<style>
    :global(.select-btn-icon) {
        transition: all 250ms;
        color: var(--color-text-body);
    }

    :global(.select-trigger){
        display: flex;
        align-items: center;
        padding: 8px 12px; 
        gap: 4px;
        border: 1px solid var(--color-neutral-50);
        border-radius: var(--radius-md);

        &[aria-expanded="true"]{
            :global(svg){
                rotate: 90deg;
            }
        }
    }


    li {
        border-radius: var(--radius-sm);
        padding: 8px;
        transition: all 250ms;
        cursor: pointer;

        &:hover {
            background-color: var(--color-neutral-50);
        }

        &[data-selected="true"] {
            background-color: var(--color-neutral-50);
        }
    }
</style>
