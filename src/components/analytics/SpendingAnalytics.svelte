<script lang="ts">
    import {analyticsStore} from "$lib/analytics.svelte";

    const analytics = analyticsStore.spendingAnalytics;
    const total = Math.max(...analytics.map(a => parseFloat(a.total)))
    // const total = analytics.reduce((acc,curr) => acc + parseFloat(curr.total),0);
</script>

<section class="space-y-2">
    <h6>Spending</h6>
    <ul>
        {#each analytics as item (item.category.id)}
            {#if parseFloat(item.total) > 0}
                {@const percent = (parseFloat(item.total)/total) * 100}
                <li class="category-title">{item.category.title}</li>
                <li style={`--percent: ${percent}%`} class="graph-bar"></li>
            {/if}
        {/each}
    </ul>
</section>

<style>
    ul{
        display: grid;
        grid-template-columns: auto 1fr;
        column-gap: 20px;
        row-gap: 24px;
        align-items: center;

        .graph-bar{
            width: var(--percent);
            height: 20px;
            border-radius: var(--radius-xs);
            background-color: var(--color-purple-500);
        }
    }
</style>