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
    import {formatDate} from "$lib/utils/date";

    type DateFn = (date: DateValue) => void;

    type Props = {
        onDateChange?: DateFn;
        value?: DateValue
    };

    // TODO: test this
    let {
        onDateChange,
        value = $bindable(today(getLocalTimeZone()))
    }: Props = $props();

    const currentDate = today(getLocalTimeZone());
    // The date is always parsed as YYYY-MM-DD regardless of the locale.
    let selectedDate = $state(currentDate.toString());

    let dateString = formatDate(toCalendarDate(value));
    $effect(() => {
        // const [year,month,day] = selectedDate.split("-").map(d => parseInt(d));
        // onChange?.(year,month,day);
    });
</script>

<input type="text" bind:value={dateString} class="date-field">

<style>
    .date-field{
        padding: 8px;
        background: var(--color-neutral-25);
        border: 1px solid var(--color-neutral-100);
        border-radius: var(--radius-sm);
        width: 100%;
    }

    /*input{*/
	/*	display: flex;*/
	/*	gap: 12px;*/
	/*	border: 1px solid var(--color-neutral-50);*/
	/*	padding: 8px 12px;*/
	/*	border-radius: var(--radius-sm);*/
    /*    outline: none;*/
    /*    transition: all 150ms;*/

    /*    &:focus{*/
    /*        border-color: var(--color-purple-500);*/
    /*    }*/
	/*}*/

</style>
