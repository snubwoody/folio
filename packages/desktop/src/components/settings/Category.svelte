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
    import Delete from "@lucide/svelte/icons/trash-2";
    import { appStore } from "$lib/state.svelte";
    import type { Category } from "$lib/lib";
    import InlineTextField from "$components/InlineTextField.svelte";
    type Props = {
        category: Category;
    };

    const { category }: Props = $props();

    const title: string = $state(category.title);
</script>

<li class="flex items-center justify-between">
    <InlineTextField value={title} onChange={(value) => appStore.editCategory(category.id,value)}/>
    <IconButton
        onclick={() => appStore.deleteCategory(category.id)}
        variant="ghost"
    >
        <Delete />
    </IconButton>
</li>