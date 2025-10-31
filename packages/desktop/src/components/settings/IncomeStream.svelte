<script lang="ts">
    import IconButton from "$components/button/IconButton.svelte";
    import Delete from "@lucide/svelte/icons/trash-2";
    import { appStore } from "$lib/state.svelte";
    import type { IncomeStream } from "$lib/lib";
    type Props = {
        stream: IncomeStream;
    };

    const { stream }: Props = $props();

    let title: string = $state(stream.title);
</script>

<li class="flex items-center justify-between">
    <div class="inline-text-field">
        <input
            type="text"
            onblur={() => appStore.editIncomeStream(stream.id, title)}
            bind:value={title}
        />
    </div>
    <IconButton
        onclick={() => appStore.deleteIncomeStream(stream.id)}
        variant="ghost"
    >
        <Delete />
    </IconButton>
</li>

<style>
    .inline-text-field {
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
