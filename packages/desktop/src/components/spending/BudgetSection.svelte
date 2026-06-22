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
    import { getBudget, totalSpent } from "$lib/api/category";
    import { categoryStore } from "$lib/stores/categories.svelte";
    import { formatMoney } from "$lib/utils/money";
    import Category from "./Category.svelte";

    // TODO: test this
    // TODO: add chevron for collapsing
    const categoriesWithoutGroup = $derived(
        categoryStore.categories.filter((c) => !c.categoryGroupId),
    );

    // TODO: add border when editing
    // - maybe add text box component or something like that
    // TODO: maybe add box shadow for transactions instead of popup?
    // - then revert to old value on failure

    // TODO: test if reactive
    // TODO: maybe extract into functions?
</script>

<section class="category-group-section">
    <div class="category-group-header border-b border-b-neutral-100">
        <p class="font-medium">Categories</p>
        <p class="font-medium">Budgeted</p>
        <p class="font-medium">Remaining</p>
    </div>
    {#each categoryStore.categoryGroups as group (group.id)}
        {@const categories = categoryStore.categories.filter(
            (c) => c.categoryGroupId === group.id,
        )}
        <ul class="category-group">
            <div class="category-group-header">
                <p>{group.title}</p>
                <p>{formatMoney("0")}</p>
                <p>{formatMoney("0")}</p>
            </div>
            <!--TODO: use composite key -->
            {#each categories as category (category.id)}
                {#await getBudget(category.id)}
                    <div class="hidden"></div>
                {:then budget}
                    <Category {category} {budget} />
                {/await}
            {/each}
        </ul>
    {/each}
    <ul class="category-group">
        <div class="category-group-header">
            <p>No group</p>
            <p>$0.00</p>
            <p>$0.00</p>
        </div>
        <!--TODO: use composite key -->
        {#each categoriesWithoutGroup as category (category.id)}
            {#await getBudget(category.id)}
                <div class="hidden"></div>
            {:then budget}
                <Category {category} {budget} />
            {/await}
        {/each}
    </ul>
</section>
