<script lang="ts">
    import { Calendar } from "bits-ui";
    import { ChevronLeft,ChevronRight } from "@lucide/svelte";
    import type { DateValue } from "@internationalized/date";

    type DateFn = (year: number, month: number, day: number) => void;

    type Props = {
        onDateChange?: DateFn;
        initialValue?: DateValue
    };

    const { onDateChange,initialValue }:Props = $props();
    // let value = $state(today(getLocalTimeZone()));
    // TODO: add value input
    let value: DateValue | undefined = $state(initialValue);
    $effect(() => {
        if (!value || !onDateChange) return;
        onDateChange(value?.year, value?.month, value?.day);
    });
</script>

<Calendar.Root
    weekdayFormat="short"
    fixedWeeks={true}
    type="single"
    bind:value
>
    {#snippet children({ months, weekdays })}
        <Calendar.Header >
            <Calendar.PrevButton
            >
                <ChevronLeft />
            </Calendar.PrevButton>
            <Calendar.Heading />
            <Calendar.NextButton
            >
                <ChevronRight />
            </Calendar.NextButton>
        </Calendar.Header>
        <div
        >
            {#each months as month, i (i)}
                <Calendar.Grid>
                    <Calendar.GridHead>
                        <Calendar.GridRow>
                            {#each weekdays as day, i (i)}
                                <Calendar.HeadCell
                                >
                                    <div>{day.slice(0, 2)}</div>
                                </Calendar.HeadCell>
                            {/each}
                        </Calendar.GridRow>
                    </Calendar.GridHead>
                    <Calendar.GridBody>
                        {#each month.weeks as weekDates, i (i)}
                            <Calendar.GridRow>
                                {#each weekDates as date, i (i)}
                                    <Calendar.Cell
                                        {date}
                                        month={month.value}
                                    >
                                        <Calendar.Day
                                        >
                                            {date.day}
                                        </Calendar.Day>
                                    </Calendar.Cell>
                                {/each}
                            </Calendar.GridRow>
                        {/each}
                    </Calendar.GridBody>
                </Calendar.Grid>
            {/each}
        </div>
    {/snippet}
</Calendar.Root>

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