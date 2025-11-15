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
<script lang="ts" generics="T">
    import { Select } from "melt/builders";
    import { type SelectOption } from "$lib/select.svelte";
    import { ChevronDown } from "@lucide/svelte";

    type Props = {
        label?: string;
        items: T[];
        toOption: (item: T) => SelectOption;
        defaultValue?: T;
        onChange?: (item: T) => void;
        class?: string
    };

    const {
        label,
        items,
        toOption,
        onChange,
        class: className,
        defaultValue
    }: Props = $props();

    let selectedOption: SelectOption | undefined = $state(
        defaultValue ? toOption(defaultValue) : undefined
    );

    const options = $derived(items.map((i) => toOption(i)));

    const onValueChange = (value?: SelectOption) => {
        const item = items.find((i) => toOption(i).value === value?.value);
        if (!item) {
            return;
        }

        selectedOption = toOption(item);
        onChange?.(item);
    };

    const select = new Select<SelectOption>({
        value: defaultValue ? toOption(defaultValue) : undefined,
        onValueChange: onValueChange
    });
</script>

<div class={`space-y-1 ${className}`}>
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
</div>
<ul {...select.content} class="popup-overlay space-y-1">
    {#each options as option (option.value)}
        {@const selected = select.isSelected(option)}
        <li {...select.getOption(option)} data-selected={selected}>
            {option.label}
        </li>
    {/each}
</ul>

<style>
    :global(.select-btn-icon) {
        transition: all 250ms;
        color: var(--color-text-body);
    }

    button[aria-expanded='true'] {
        :global(.select-btn-icon){
            rotate: 90deg;
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
