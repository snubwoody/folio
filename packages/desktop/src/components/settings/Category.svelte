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

    let title: string = $state(category.title);
</script>

<li class="flex items-center justify-between">
    <InlineTextField value={title} onChange={(value)=>appStore.editCategory(category.id,value)}/>
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
