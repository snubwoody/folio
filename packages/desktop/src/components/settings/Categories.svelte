<script lang="ts">
    import IconButton from "$components/button/IconButton.svelte";
    import Plus from "@lucide/svelte/icons/plus";
    import { appStore } from "$lib/state.svelte";
    import Category from "./Category.svelte";

    let categories = $derived.by(() => {
        return appStore.categories
            .toSorted(
                (a, b) =>
                    new Date(a.createdAt).getTime() -
                    new Date(b.createdAt).getTime(),
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
