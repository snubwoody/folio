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
