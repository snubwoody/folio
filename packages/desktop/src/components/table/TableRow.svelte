<script lang="ts">
    import type {Snippet} from "svelte";
    import type {HTMLAttributes} from "svelte/elements";

    interface Props extends HTMLAttributes<HTMLDivElement>{
        children?: Snippet
    }

    const { children,...rest }:Props = $props();
</script>

<div class="table-row" {...rest}>
    {@render children?.()}
</div>
