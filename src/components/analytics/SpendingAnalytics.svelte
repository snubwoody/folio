<script lang="ts">
    import { appStore } from "$lib/state.svelte";

    const analytics = appStore.spendingAnaltics;
    const total = Math.max(...analytics.map(a => parseFloat(a.total)));

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
        "var(--color-purple-900)",
    ];

    console.log(analytics);
</script>

<section class="space-y-2">
    <h6>Spending</h6>
    <ul>
        {#each analytics as item,index (item.category.id)}
            {#if parseFloat(item.total) > 0}
                {@const percent = (parseFloat(item.total)/total) * 100}
                {@const color = purpleShades[(index+2) % purpleShades.length]}
                <li class="category-title">{item.category.title}</li>
                <li style={`--percent: ${percent}%;--bar-color:${color}`} class="graph-bar"></li>
            {/if}
        {/each}
    </ul>
</section>

<style>
    ul{
        display: grid;
        grid-template-columns: auto 1fr;
        gap: 24px 20px;
        align-items: center;

        .graph-bar{
            width: var(--percent);
            height: 20px;
            border-radius: var(--radius-xs);
            background-color: var(--bar-color);
        }
    }
</style>