<script lang="ts">
	import { createDateField } from "@melt-ui/svelte";

	type Props = {
		onChange?: (year: number,month: number,day: number) => void
	}

    const { onChange }: Props = $props();
	const {
	    elements: { field, segment, label },
	    states: { segmentContents },
	} = createDateField({ locale: "en-US",onValueChange:({ next }) => {
	    if(next && onChange){
	        onChange(next.year,next.month,next.day);
	    }
	    return next;
	} });
</script>

<div class="flex h-full w-full flex-col gap-1">
    <span {...$label} use:label class="text-sm text-text-muted">Date</span>
    <div {...$field} use:field>
        {#each $segmentContents as seg (seg)}
            <div {...$segment(seg.part)} use:segment>
            {seg.value}
            </div>
        {/each}
    </div>
</div>

<style>
	[data-melt-datefield-field]{
		display: flex;
		gap: 12px;
		border: 1px solid var(--color-neutral-50);
		padding: 8px 12px;
		border-radius: var(--radius-sm);
	}

	[data-melt-datefield-segment]{
		outline: none;

		&:focus{
			position: relative;

			/* background-color: aqua; */
			&::after{
				content: "";
				position: absolute;
				height: 1px;
				width: 100%;
				bottom: 2px;
				left: 0;
				background-color: var(--color-surface-primary);
			}
		}
	}
</style>
