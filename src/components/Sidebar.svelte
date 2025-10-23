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

<script>
    import IconButton from "$components/button/IconButton.svelte";
    import Home from "@lucide/svelte/icons/house";
    import Chart from "@lucide/svelte/icons/chart-no-axes-combined";
    import Settings from "@lucide/svelte/icons/settings";
</script>

<nav>
    <IconButton href="/" variant="ghost">
        <Home size="20"/>
    </IconButton>
    <IconButton href="/analytics" variant="ghost">
        <Chart size="20"/>
    </IconButton>
    <IconButton variant="neutral" class="mt-auto">
        <Settings size="20" class="w-3"/>
    </IconButton>
</nav>

<style>
	nav{
		width: fit-content;
		height: 100%;
		display: flex;
		flex-direction: column;
		padding: 12px;
		background-color: var(--color-neutral-25);
	}
</style>
