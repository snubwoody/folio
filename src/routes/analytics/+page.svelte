<script lang="ts">
    import BudgetOverview from "$components/analytics/BudgetOverview.svelte";
    import BudgetSection from "$components/analytics/BudgetSection.svelte";
    import IncomeAnalytics from "$components/analytics/IncomeAnalytics.svelte";
    import SpendingAnalytics from "$components/analytics/SpendingAnalytics.svelte";
</script>

<main class="space-y-5 px-4 py-7">
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
