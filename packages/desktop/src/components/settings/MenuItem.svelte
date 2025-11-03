<script lang="ts">
    import { type Snippet } from "svelte";

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