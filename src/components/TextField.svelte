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
		label?: string,
		value: string,
		placeholder?: string
	}

	let {
	    label,
	    placeholder,
	    value = $bindable(),
	}: Props = $props();
</script>

<div class="flex flex-col gap-1">
    {#if label}
        <p class="text-sm text-text-muted">{label}</p>
    {/if}
	<input bind:value type="text" {placeholder}>
</div>

<style>
	input{
		border: 1px solid var(--color-neutral-50);
		padding: 8px 12px;
		border-radius: var(--radius-md);
		transition: all 150ms;
		outline: none;

		&:focus{
			border-color: var(--color-purple-500);
		}
	}
</style>
