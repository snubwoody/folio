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
	type Props = {
	    onChange?: (year: number,month: number,day: number) => void
	};

    const { onChange }: Props = $props();
    // The date is always parsed as YYYY-MM-DD regardless of the locale.
    let selectedDate: string | undefined= $state(undefined);

    $effect(() => {
        if (!selectedDate) return;
        const [year,month,day] = selectedDate.split("-").map(d => parseInt(d));
        onChange?.(year,month,day);
    });
</script>

<label class="flex h-full w-full flex-col gap-1">
    <p class="text-sm text-text-muted">Date</p>
    <input bind:value={selectedDate} type="date" name="" id="">
</label>

<style>
    input{
		display: flex;
		gap: 12px;
		border: 1px solid var(--color-neutral-50);
		padding: 8px 12px;
		border-radius: var(--radius-sm);
        outline: none;
        transition: all 150ms;

        &:focus{
            border-color: var(--color-purple-500);
        }
	}

    /* Hide default calendar button */
    input[type="date"]::-webkit-inner-spin-button,
    input[type="date"]::-webkit-calendar-picker-indicator {
        display: none;
        appearance: none;
    }
</style>
