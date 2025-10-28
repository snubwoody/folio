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
    import { CalendarDays, ChevronLeft, ChevronRight } from "@lucide/svelte";
    import { DatePicker } from "bits-ui";
    import { type DateValue } from "@internationalized/date";

    type Props = {
        onDateChange?: (year: number, month: number, day: number) => void;
    };

    const { onDateChange }: Props = $props();
    let value: DateValue | undefined = $state();
    $effect(() => {
        if (!value || !onDateChange) return;
        onDateChange(value?.year, value?.month, value?.day);
    });
</script>

<DatePicker.Root bind:value>
    <DatePicker.Label />
    <DatePicker.Input>
        <DatePicker.Trigger>
            <button
                id="calendar-button"
                class="icon-btn icon-btn-ghost icon-btn-medium"
            >
                <CalendarDays />
            </button>
        </DatePicker.Trigger>
    </DatePicker.Input>
    <DatePicker.Content>
        <DatePicker.Calendar>
            {#snippet children({ months, weekdays })}
                <DatePicker.Header>
                    <DatePicker.PrevButton>
                        <button
                            id="calendar-button"
                            class="icon-btn icon-btn-grey icon-btn-medium"
                        >
                            <ChevronLeft />
                        </button>
                    </DatePicker.PrevButton>
                    <DatePicker.Heading />
                    <DatePicker.NextButton>
                        <button
                            id="calendar-button"
                            class="icon-btn icon-btn-ghost icon-btn-medium"
                        >
                            <ChevronRight />
                        </button>
                    </DatePicker.NextButton>
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

<style>

    :global([data-calendar-root]) {
        padding: 12px;
        border-radius: var(--radius-md);
        box-shadow: var(--shadow-md);
        background-color: white;
    }

    :global([data-calendar-header]) {
        display: flex;
        gap: 4px;
        justify-content: space-between;
        align-items: center;
    }

    :global([data-calendar-day]) {
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

        &[data-disabled] {
            color: var(--color-text-muted);
            background: transparent;
        }
    }
</style>
