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

<script lang="ts" generics="T">
    import { Select } from "melt/builders";
    import IconButton from "$components/button/IconButton.svelte";
    import ChevronDown from "@lucide/svelte/icons/chevron-down";
    import type { SelectOption } from "$lib/select.svelte";

    type Props = {
        items: T[];
        toOption: (item: T) => SelectOption;
        defaultValue?: T;
        onChange?: (item: T) => void;
    };

    const { items, toOption, onChange, defaultValue }: Props = $props();

    const options = $derived(items.map((i) => toOption(i)));

    const onValueChange = (value?: SelectOption) => {
        const item = items.find((i) => toOption(i).value === value?.value);
        if (!item) {
            return;
        }

        onChange?.(item);
    };

    const select = new Select<SelectOption>({
        value: defaultValue ? toOption(defaultValue) : undefined,
        onValueChange: onValueChange
    });
</script>

<div {...select.trigger} class="space-y-1">
    <IconButton variant="ghost">
        <ChevronDown />
    </IconButton>
</div>
<ul {...select.content} class="popup-overlay space-y-1 w-fit!">
    {#each options as option (option.value)}
        {@const selected = select.isSelected(option)}
        <li {...select.getOption(option)} data-selected={selected}>
            {option.label}
        </li>
    {/each}
</ul>

<style>
    li {
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
