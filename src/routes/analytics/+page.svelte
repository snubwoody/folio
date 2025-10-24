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
    import BudgetOverview from "$components/analytics/BudgetOverview.svelte";
    import BudgetSection from "$components/analytics/BudgetSection.svelte";
    import IncomeAnalytics from "$components/analytics/IncomeAnalytics.svelte";
    import SpendingAnalytics from "$components/analytics/SpendingAnalytics.svelte";
</script>

<main class="space-y-5 px-4 py-7 overflow-y-auto">
    <BudgetOverview/>
    <div class="divider"></div>
    <IncomeAnalytics/>
    <div class="divider"></div>
    <SpendingAnalytics/>
    <div class="divider"></div>
    <BudgetSection/>
</main>

<style>
    .divider{
        width: 100%;
        height: 1px;
        border-radius: var(--radius-sm);
        background-color: var(--color-neutral-50);
    }
</style>
