<script lang="ts">
    import { Popover } from "bits-ui";

    const { children,class:userClass }: Popover.ContentProps = $props();
</script>

<Popover.Portal>
    <Popover.Content class={`popup-overlay ${userClass??""}`}>
        {@render children?.()}
    </Popover.Content>
</Popover.Portal>
