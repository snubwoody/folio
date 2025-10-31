<script lang="ts">
    import IconButton from "$components/button/IconButton.svelte";
    import Delete from "@lucide/svelte/icons/trash-2";
    import { appStore } from "$lib/state.svelte";
    import type { Category } from "$lib/lib";
    type Props = {
        category: Category;
    };

    const { category }: Props = $props();

    let title: string = $state(category.title);
</script>

<li class="flex items-center justify-between">
    <!-- <p>{category.title}</p> -->
    <div class="inline-text-field">
        <input
            type="text"
            onblur={() => appStore.editCategory(category.id, title)}
            bind:value={title}
        />
    </div>
    <IconButton
        onclick={() => appStore.deleteCategory(category.id)}
        variant="ghost"
    >
        <Delete />
    </IconButton>
</li>

<style>
    :global(.inline-text-field) {
        position: relative;

        &::after {
            position: absolute;
            content: "";
            height: 1px;
            bottom: -4px;
            left: 0;
            width: 100%;
            background-color: transparent;
            transition: all 250ms;
        }

        &:focus-within {
            &::after {
                background-color: black;
            }
        }

        input {
            outline: none;
        }
    }
</style>
