<script lang="ts">
    import { Calendar } from "bits-ui";
    import { ChevronLeft,ChevronRight } from "@lucide/svelte";
    import { getLocalTimeZone, today, type DateValue } from "@internationalized/date";
    import { TextButton } from "$components/button";
    import DateField from "./DateField.svelte";

    type DateFn = (date: DateValue) => void;

    type Props = {
        onDateChange?: DateFn;
        value?: DateValue
    };

    // TODO: default today
    let {
        value = $bindable(today(getLocalTimeZone())),
        onDateChange
    }:Props = $props();

    function updateDate(date: DateValue | undefined){
        if (!date || !onDateChange) return;
        onDateChange(date);
    }

    function setToday(){
        updateDate(today(getLocalTimeZone()));
    }

    // $effect(()=>{
    //     updateDate(value)
    //     console.log("Updating date")
    // })
    // TODO: update after a certain timeout, onblur
    // TODO: focus trap
    // TODO: red border
    // TODO: ARIA label for date field or properties
    // TODO: bind value
    // FIXME: not updating date, test
    let dateFieldValue = value?.toDate(getLocalTimeZone()).toString();
    // TODO: add enter and on blur

    // TODO: close on enter instead of click?
    // TODO: test has date field
</script>

<Calendar.Root
    weekdayFormat="short"
    fixedWeeks={true}
    type="single"
    onValueChange={updateDate}
    class="calendar"
    data-testid="calendar"
    onblur={(e)=> console.log("Blurred")}
    bind:value
>
    {#snippet children({ months, weekdays })}
        <DateField bind:value/>
        <Calendar.Header class="flex items-center justify-between px-0.5">
            <Calendar.Heading  class="font-semibold"/>
            <div class="flex items-center gap-1">
                <TextButton class="font-semibold" onclick={setToday}>Today</TextButton>
                <div class="flex items-center gap-0.5">
                    <Calendar.PrevButton class="icon-btn icon-btn-primary-icon icon-btn-medium">
                        <ChevronLeft strokeWidth="3"/>
                    </Calendar.PrevButton>
                    <Calendar.NextButton class="icon-btn icon-btn-primary-icon icon-btn-medium">
                        <ChevronRight strokeWidth="3"/>
                    </Calendar.NextButton>
                </div>
            </div>
        </Calendar.Header>
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
    {/snippet}
</Calendar.Root>

<style>
    .date-field{
        padding: 8px;
        background: var(--color-neutral-25);
        border: 1px solid var(--color-neutral-100);
        border-radius: var(--radius-sm);
        width: 100%;
    }
</style>
