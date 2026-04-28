<script lang="ts">
    import { Checkbox } from "bits-ui";
    import { Check } from "@lucide/svelte";

    type Props = {
        onChecked?: (value: boolean) => void,
        checked?: boolean
    };

    let {
        onChecked,
        checked = $bindable(false)
    }:Props = $props();
</script>

<Checkbox.Root bind:checked onCheckedChange={onChecked} class="checkbox">
    {#snippet children({ checked })}
        {#if checked}
            <Check size="12" strokeWidth="3"/>
        {/if}
    {/snippet}
</Checkbox.Root>

<style>
    :global(.checkbox) {
        width: 16px;
        height: 16px;
        display: flex;
        align-items: center;
        justify-content: center;
        border: 1px solid var(--color-neutral-100);
        transition: all 250ms ease;
        border-radius: var(--radius-xs);

        &:hover {
            border-color: var(--color-purple-300);
        }

        :global(&[data-state="checked"],&[data-state="intermediate"]) {
            background: var(--color-surface-primary);
            border-color: var(--color-surface-primary);
            color: white;
        }
    }
</style>