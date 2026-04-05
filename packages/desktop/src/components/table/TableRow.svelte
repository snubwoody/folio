<script lang="ts">
    import type {Snippet} from "svelte";

    interface Props{
        children?: Snippet
    }

    const { children }:Props = $props();
</script>

<div class="table-row">
    {@render children?.()}
</div>

<style>
    .table-row{
        display: flex;
        border-bottom: 1px solid var(--color-neutral-50);
    }
</style>