<!--
@component
Displays content in a portal, triggered by a button.

# Example
```svelte
<script>
    import {Popover,PopoverTrigger,PopoverContent} from "$components/popover";
</script>

<Popover>
    <PopoverTrigger>Open popover</PopoverTrigger>
    <PopoverContent>
        Popover content...
    </PopoverContent/>
</Popover>
```
-->
<script lang="ts">
    import { Popover } from "bits-ui";

    let {
        children,
        open = $bindable(false),
        ...rest
    }: Popover.RootProps = $props();
</script>

<Popover.Root bind:open={open} {...rest}>
    {@render children?.()}
</Popover.Root>