<script lang="ts">
	import {createDatePicker,} from "@melt-ui/svelte";
    import { scale } from "svelte/transition";

	type Props = {
		onDateChange?: (year?: number,month?: number,day?: number) => void;
	}

	const {onDateChange}: Props = $props();

	const {
		elements: {
			calendar,
			cell,
			content,
			grid,
			heading,
			nextButton,
			prevButton,
			trigger,
		},
		states: { months, headingValue, weekdays, segmentContents,open,value },
		helpers: { isDateDisabled, isDateUnavailable }
	} = createDatePicker({
		onValueChange: ({next}) => {
			if (onDateChange){
				onDateChange(next?.year,next?.month,next?.day)
			}
			return next;
		}
	});
</script>

<button {...$trigger} use:trigger class="icon-btn icon-btn-grey icon-btn-medium">
	<i class="ph ph-calendar"></i>
</button>
{#if $open}
	<div transition:scale={{start:0.8}} {...$content} use:content>
	<div {...$calendar} use:calendar>
		<header class="calendar-header">
		<button class="icon-btn icon-btn-medium icon-btn-primary" {...$prevButton} use:prevButton>
			<i class="ph ph-caret-left"></i>
		</button>
		<div {...$heading} use:heading>
			{$headingValue}
		</div>
		<button class="icon-btn icon-btn-medium icon-btn-primary" {...$nextButton} use:nextButton>
			<i class="ph ph-caret-right"></i>
		</button>
		</header>
		{#each $months as month}
		<table {...$grid} use:grid>
			<thead aria-hidden="true">
			<tr>
				{#each $weekdays as day}
				<th>
					{day}
				</th>
				{/each}
			</tr>
			</thead>
			<tbody>
			{#each month.weeks as days}
				<tr>
				{#each days as date}
					<td
					role="gridcell"
					aria-disabled={$isDateDisabled(date) || $isDateUnavailable(date)}>
					<div {...$cell(date, month.value)} use:cell>
						{date.day}
					</div>
					</td>
				{/each}
				</tr>
			{/each}
			</tbody>
		</table>
		{/each}
	</div>
	</div>
{/if}

<style>
	.calendar-header{
		display: flex;
		padding: 8px 0px;
		align-items: center;
		justify-content: space-between;
	}

	[data-melt-calendar]{
		padding: 12px;
		border-radius: var(--radius-md);
		box-shadow: var(--shadow-md);
		background-color: white;
	}

	[data-melt-calendar-heading]{
		display: flex;
		flex-direction: column;
		gap: 4px;
		align-items: center;
	}

	[data-melt-calendar-cell]{
		display: grid;
		place-items: center;
		padding: 8px;
		border-radius: var(--radius-sm);
		transition: all 250ms;
		cursor: pointer;

		&[data-today] {
			outline: 1px solid var(--color-purple-500);
		}

		&[data-selected]{
			background-color: var(--color-surface-primary);
			color: var(--color-white);
		}
	}
</style>