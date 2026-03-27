<script lang="ts">
    import { Calendar } from "bits-ui";
    import { ChevronLeft,ChevronRight } from "@lucide/svelte";
    import { getLocalTimeZone, today, type DateValue } from "@internationalized/date";
    import {TextButton} from "$components/button";

    type DateFn = (date: DateValue) => void;

    type Props = {
        onDateChange?: DateFn;
        value?: DateValue
    };

    let {
        value = $bindable(),
        onDateChange
    }:Props = $props();

    function updateDate(date: DateValue | undefined){
        if (!date || !onDateChange) return;
        onDateChange(date);
    }

    function setToday(){
        // value = today(getLocalTimeZone());
        updateDate(today(getLocalTimeZone()));
    }
</script>

<Calendar.Root
    weekdayFormat="short"
    fixedWeeks={true}
    type="single"
    onValueChange={updateDate}
    class="calendar"
    bind:value
>
    {#snippet children({ months, weekdays })}
        <Calendar.Header class="flex items-center justify-between px-0.5">
            <Calendar.Heading  class="font-semibold"/>
            <div class="flex items-center gap-1">
                <!--TODO: test this-->
                <TextButton class="font-semibold" onclick={setToday}>Today</TextButton>
                <div class="flex items-center gap-0.5">
                    <Calendar.PrevButton class="icon-btn icon-btn-primaryIcon icon-btn-medium">
                        <ChevronLeft strokeWidth="3"/>
                    </Calendar.PrevButton>
                    <Calendar.NextButton class="icon-btn icon-btn-primaryIcon icon-btn-medium">
                        <ChevronRight strokeWidth="3"/>
                    </Calendar.NextButton>
                </div>
            </div>
        </Calendar.Header>
        <div>
            {#each months as month, i (i)}
                <Calendar.Grid>
                    <Calendar.GridHead>
                        <Calendar.GridRow>
                            {#each weekdays as day, i (i)}
                                <Calendar.HeadCell class="calendar-weekday font-normal"
                                >
                                    {day.slice(0, 2)}
                                </Calendar.HeadCell>
                            {/each}
                        </Calendar.GridRow>
                    </Calendar.GridHead>
                    <Calendar.GridBody>
                        {#each month.weeks as weekDates, i (i)}
                            <Calendar.GridRow>
                                {#each weekDates as date, i (i)}
                                    <Calendar.Cell {date} month={month.value}>
                                        <Calendar.Day class="calendar-day">
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

    /* :global([data-calendar-root]) {
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
    } */
</style>