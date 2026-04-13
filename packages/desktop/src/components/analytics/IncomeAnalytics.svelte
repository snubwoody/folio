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
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { Analytic } from "$lib/types";

    let analytics:Analytic[] = $state([]);
    $effect(() => {
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        const _ = transactionStore.transactions;
        invoke<Analytic[]>("analytics").then(val => {
            // TODO: this might update it twice
            analytics = val.filter(a => a.category.isIncomeStream === true);
        });
    });
    const total = $derived.by(() => analytics.reduce((acc,item) => acc + parseFloat(item.total),0));

    const blueShades = [
        "var(--color-blue-50)",
        "var(--color-blue-100)",
        "var(--color-blue-200)",
        "var(--color-blue-300)",
        "var(--color-blue-400)",
        "var(--color-blue-500)",
        "var(--color-blue-600)",
        "var(--color-blue-700)",
        "var(--color-blue-800)",
        "var(--color-blue-900)"
    ];

    // TODO: add no income streams found
    // TODO: overflow x
</script>

<section class="space-y-2">
    <h6>Income streams</h6>
    <ul>
        {#each analytics as item,index (item.category.id)}
            {#if parseFloat(item.total) > 0}
                {@const percent = (parseFloat(item.total)/total) * 100}
                {@const color = blueShades[(index+5) % blueShades.length]}
                <li style={`--percent: ${percent}; --bar-color: ${color}`}>
                    <div style={`--percent: ${percent}; --bar-color: ${color}`} class="graph-bar mb-1.5"></div>
                    <p class="category-title">{item.category.title}</p>
                    <p class="text-sm text-text-muted">{percent.toFixed(2)}%</p>
                </li>
            {/if}
        {/each}
    </ul>
</section>

<style>
    ul{
        display: flex;
        grid-template-columns: auto 1fr;
        gap: 24px;
        align-items: center;

        li{
            width: 100%;
            flex: var(--percent);
        }

        .graph-bar{
            width: 100%;
            height: 20px;
            border-radius: var(--radius-xs);
            background-color: var(--bar-color);
        }
    }
</style>