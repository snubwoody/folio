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
    import IconButton from "$components/button/IconButton.svelte";
    import Plus from "@lucide/svelte/icons/plus";
    import { appStore } from "$lib/state.svelte";
    import Category from "./Category.svelte";

    const categories = $derived.by(() => {
        return (appStore.categories ?? [])
            .toSorted(
                (a, b) =>
                    new Date(a.createdAt).getTime() -
                    new Date(b.createdAt).getTime()
            )
            .reverse();
    });
</script>

<div class="space-y-4">
    <header class="space-y-0.5">
        <div class="flex items-center justify-between">
            <h6>Categories</h6>
            <IconButton
                onclick={() => appStore.createCategory()}
                variant="neutral"
            >
                <Plus />
            </IconButton>
        </div>
        <p class="text-sm">Categories are used for organising expenses.</p>
    </header>
    <ul class="space-y-2">
        {#each categories as category (category.id)}
            <Category {category} />
        {/each}
    </ul>
</div>
