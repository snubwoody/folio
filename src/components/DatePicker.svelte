<!--
Copyright (C) 2025 Wakunguma Kalimukwa

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
-->
<script lang="ts">
    import { createDatePicker } from "@melt-ui/svelte";
    import { CalendarDays } from "@lucide/svelte";
    import { DatePicker, Portal } from "bits-ui";
    import { scale } from "svelte/transition";
    import { CalendarDate } from "@internationalized/date";
    import Calendar from "$components/date/Calendar.svelte";

    type Props = {
        onDateChange?: (year: number, month: number, day: number) => void;
    };

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
        defaultValue: new CalendarDate(2025, 10, 10),
        onValueChange: ({ next }) => {
            if (onDateChange && next) {
                onDateChange(next.year, next.month, next.day);
            }
            return next;
        },
    });

    let disabled = $state(false);
</script>

<DatePicker.Root>
    <DatePicker.Label />
    <DatePicker.Input>
        {#snippet children({ children })}
            <DatePicker.Trigger>
                <button
                    id="calendar-button"
                    class="icon-btn icon-btn-grey icon-btn-medium relative"
                    onclick={() => (disabled = !disabled)}
                >
                    <CalendarDays />
                </button>
            </DatePicker.Trigger>
        {/snippet}
    </DatePicker.Input>
    <DatePicker.Content>
        <DatePicker.Calendar>
            {#snippet children({ months, weekdays })}
                <DatePicker.Header>
                    <DatePicker.PrevButton />
                    <DatePicker.Heading />
                    <DatePicker.NextButton />
                </DatePicker.Header>
                {#each months as month (month)}
                    <DatePicker.Grid>
                        <DatePicker.GridHead>
                            <DatePicker.GridRow>
                                {#each weekdays as day, index (index)}
                                    <DatePicker.HeadCell>
                                        {day}
                                    </DatePicker.HeadCell>
                                {/each}
                            </DatePicker.GridRow>
                        </DatePicker.GridHead>
                        <DatePicker.GridBody>
                            {#each month.weeks as weekDates, index (index)}
                                <DatePicker.GridRow>
                                    {#each weekDates as date, index (index)}
                                        <DatePicker.Cell
                                            {date}
                                            month={month.value}
                                        >
                                            <DatePicker.Day />
                                        </DatePicker.Cell>
                                    {/each}
                                </DatePicker.GridRow>
                            {/each}
                        </DatePicker.GridBody>
                    </DatePicker.Grid>
                {/each}
            {/snippet}
        </DatePicker.Calendar>
    </DatePicker.Content>
</DatePicker.Root>

<!--
<button
    id="calendar-button"
    class="icon-btn icon-btn-grey icon-btn-medium relative"
    onclick={() => (disabled = !disabled)}
>
    <i class="ph ph-calendar"></i>
</button>

{#if disabled}
    <div class="absolute">
        <Portal {disabled} to="#calendar-button">This is a portal</Portal>
    </div>
{/if} -->

<!---
{#if $open}
    <div
        transition:scale={{ start: 0.8 }}
        {...$content}
        use:content
        class="absolute z-100"
    >
        <div {...$calendar} use:calendar>
            <header class="calendar-header">
                <button
                    class="icon-btn icon-btn-medium icon-btn-neutral"
                    {...$prevButton}
                    use:prevButton
                >
                    <i class="ph ph-caret-left"></i>
                </button>
                <div {...$heading} use:heading>
                    {$headingValue}
                </div>
                <button
                    class="icon-btn icon-btn-medium icon-btn-neutral"
                    {...$nextButton}
                    use:nextButton
                >
                    <i class="ph ph-caret-right"></i>
                </button>
            </header>
            {#each $months as month (month.value.month)}
                <table {...$grid} use:grid>
                    <thead aria-hidden="true">
                        <tr>
                            {#each $weekdays as day, i (i)}
                                <th>{day}</th>
                            {/each}
                        </tr>
                    </thead>
                    <tbody>
                        {#each month.weeks as days, i (i)}
                            <tr>
                                {#each days as date (date)}
                                    <td
                                        role="gridcell"
                                        aria-disabled={$isDateDisabled(date) ||
                                            $isDateUnavailable(date)}
                                    >
                                        <div
                                            {...$cell(date, month.value)}
                                            use:cell
                                        >
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
{/if} -->

<style>
    .calendar-header {
        display: flex;
        padding: 8px 0;
        align-items: center;
        justify-content: space-between;
    }

    [data-disabled] {
        color: var(--color-text-muted);
    }

    [data-calendar-root] {
        padding: 12px;
        border-radius: var(--radius-md);
        box-shadow: var(--shadow-md);
        background-color: white;
    }

    [data-melt-calendar-heading] {
        display: flex;
        flex-direction: column;
        gap: 4px;
        align-items: center;
    }

    [data-calendar-day] {
        display: grid;
        place-items: center;
        padding: 8px;
        border-radius: var(--radius-sm);
        transition: all 250ms;
        cursor: pointer;
        user-select: none;

        &:hover {
            background-color: var(--color-purple-100);
        }

        &[data-today] {
            outline: 1px solid var(--color-purple-500);
        }

        &[data-selected] {
            background-color: var(--color-surface-primary);
            color: var(--color-white);
        }
    }
</style>
