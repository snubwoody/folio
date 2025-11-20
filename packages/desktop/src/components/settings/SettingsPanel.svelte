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
    import IncomeStreams from "./IncomeStreams.svelte";
    import General from "./General.svelte";
    import Categories from "./Categories.svelte";
    import MenuItem from "./MenuItem.svelte";
    let selectedIndex = $state(0);
</script>

<div aria-label="Settings panel" class="settings-panel">
    <aside>
        <MenuItem bind:selectedIndex index={0}>General</MenuItem>
        <MenuItem bind:selectedIndex index={1}>Categories</MenuItem>
        <MenuItem bind:selectedIndex index={2}>Income streams</MenuItem>
    </aside>
    <main class="overflow-y-auto">
        {#if selectedIndex === 0}
             <General/>
        {:else if selectedIndex === 1}
            <Categories />
        {:else if selectedIndex === 2}
            <IncomeStreams />
        {/if}
    </main>
</div>

<style>
    .settings-panel {
        display: grid;
        grid-template-columns: auto 1fr;
        height: 100%;
    }

    main {
        width: 100%;
        padding: 40px;
    }

    aside {
        display: flex;
        width: 100%;
        flex-direction: column;
        gap: 8px;
        padding: 24px 16px;
        background: var(--color-neutral-25);
        height: 100%;
    }
</style>
