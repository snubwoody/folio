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
    import { calculateSpendingAnalytics } from "$lib/analytics";
    import { appStore } from "$lib/state.svelte";

    const analytics = $derived.by(() => calculateSpendingAnalytics(appStore.expenses));
    const total = $derived(Math.max(...analytics.map(a => a.total)));

    const purpleShades = [
        "var(--color-purple-50)",
        "var(--color-purple-100)",
        "var(--color-purple-200)",
        "var(--color-purple-300)",
        "var(--color-purple-400)",
        "var(--color-purple-500)",
        "var(--color-purple-600)",
        "var(--color-purple-700)",
        "var(--color-purple-800)",
        "var(--color-purple-900)"
    ];
</script>

<section class="space-y-2">
    <h6>Spending</h6>
    <div class="spending-graph">
        {#each analytics as item,index (item.category.id)}
            {#if item.total > 0} <!--Prevent zero division errors-->
                {@const percent = (item.total/total) * 100}
                {@const color = purpleShades[(index+2) % purpleShades.length]}
                <div class="category-title">{item.category.title}</div>
                <div style={`--percent: ${percent}%;--bar-color:${color}`} class="graph-bar"></div>
            {/if}
        {/each}
        <div class="vertical-bar"></div>
    </div>
</section>

<style>
    .spending-graph {
        display: grid;
        grid-template-columns: auto 1fr;
        gap: 24px 20px;
        align-items: center;
        position: relative;
        counter-reset: bar;

        .graph-bar{
            width: var(--percent);
            height: 20px;
            border-radius: var(--radius-xs);
            background-color: var(--bar-color);
        }
    }

    .vertical-bar{
        counter-increment: bar;
        position: absolute;
        width: 2px;
        height: 100%;
        top: 0;
        left: 20%;
        background: var(--color-neutral-200);
        grid-column-start: 2;
    }

</style>