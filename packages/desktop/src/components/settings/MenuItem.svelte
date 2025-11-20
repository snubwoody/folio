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
    import type { Snippet } from "svelte";

    type Props = {
        children: Snippet,
        index: number,
        selectedIndex:number
    };

    let {
        children,
        index,
        selectedIndex = $bindable()
    }:Props = $props();

    const selected = $derived(index === selectedIndex);
</script>

<button onclick={() => selectedIndex = index} data-selected={selected}>
    {@render children()}
</button>

<style>
    button{
        display: flex;
        padding: 4px 12px;
        transition: all 250ms;
        border-radius: var(--radius-sm);

        &:hover{
            background: var(--color-neutral-50);
        }

        &[data-selected="true"]{
            background: var(--color-surface-primary);
            color: var(--color-white);
        }
    }
</style>