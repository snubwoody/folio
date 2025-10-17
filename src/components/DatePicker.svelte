<script lang="ts">
	import { createDatePicker, } from "@melt-ui/svelte";
    import { scale } from "svelte/transition";
    import {CalendarDate} from "@internationalized/date";

	type Props = {
		onDateChange?: (year: number,month: number,day: number) => void;
	}

	const { onDateChange }: Props = $props();

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
	    states: { months, headingValue, weekdays, open },
	    helpers: { isDateDisabled, isDateUnavailable },
	} = createDatePicker({
        fixedWeeks: true,
        defaultValue: new CalendarDate(2025,10,10),
	    onValueChange: ({ next }) => {
	        if (onDateChange && next){
	            onDateChange(next.year,next.month,next.day);
	        }
	        return next;
	    },
	});
</script>

<button {...$trigger} use:trigger class="icon-btn icon-btn-grey icon-btn-medium">
	<i class="ph ph-calendar"></i>
</button>
{#if $open}
	<div transition:scale={{ start:0.8 }} {...$content} use:content class="absolute z-100">
	    <div {...$calendar} use:calendar>
		<header class="calendar-header">
            <button class="icon-btn icon-btn-medium icon-btn-neutral" {...$prevButton} use:prevButton>
                <i class="ph ph-caret-left"></i>
            </button>
            <div {...$heading} use:heading>
                {$headingValue}
            </div>
            <button class="icon-btn icon-btn-medium icon-btn-neutral" {...$nextButton} use:nextButton>
                <i class="ph ph-caret-right"></i>
            </button>
		</header>
		{#each $months as month (month.value.month)}
            <table {...$grid} use:grid>
                <thead aria-hidden="true">
                <tr>
                    {#each $weekdays as day,i (i)}
                    <th>{day}</th>
                    {/each}
                </tr>
                </thead>
                <tbody>
                {#each month.weeks as days,i (i)}
                    <tr>
                    {#each days as date (date)}
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
		padding: 8px 0;
		align-items: center;
		justify-content: space-between;
	}

    [data-disabled]{
        color: var(--color-text-muted);
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
        user-select: none;

        &:hover{
            background-color: var(--color-purple-100);
        }

		&[data-today] {
			outline: 1px solid var(--color-purple-500);
		}

		&[data-selected]{
			background-color: var(--color-surface-primary);
			color: var(--color-white);
		}
	}
</style>
