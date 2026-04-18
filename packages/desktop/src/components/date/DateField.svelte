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
    import {type DateValue, getLocalTimeZone, toCalendarDate, today} from "@internationalized/date";
    import {formatDate, parseDate} from "$lib/utils/date";

    type DateFn = (date: DateValue) => void;

    type Props = {
        /**
         * The date change event fires whenever the date is changed, which occurs when
         * the date field loses focus or when the user presses the Enter key.
         */
        onDateChange?: DateFn;
        value?: DateValue
    };

    let {
        onDateChange,
        value = $bindable(today(getLocalTimeZone()))
    }: Props = $props();

    let dateString = $state(formatDate(toCalendarDate(value)));

    // TODO: add accessible, aria-invalid?
    // TODO: focus trap
    // TODO: if i update value it might cause a circular dependency
    const updateDate = async(val:string) => {
        const date = await parseDate(val);
        dateString = formatDate(toCalendarDate(date))
        value = date;
        onDateChange?.(value);
    }

    const handleKeyPress = (event: KeyboardEvent) => {
        if (event.key !== "Enter") return
        (event.target as HTMLElement).blur();
    }
</script>

<input
    type="text"
    bind:value={dateString}
    onkeydown={handleKeyPress}
    onblur={(e) => updateDate(e.currentTarget.value)}
    class="date-field">

<style>
    .date-field{
        padding: 8px;
        background: var(--color-neutral-25);
        border: 1px solid var(--color-neutral-100);
        border-radius: var(--radius-sm);
        width: 100%;
        outline-color: var(--color-purple-500);
    }
</style>
