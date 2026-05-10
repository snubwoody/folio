<script lang="ts">
    import type { SpendingAnalytic } from "$lib/analytics";
    import { formatMoney } from "$lib/utils/money";

    type Props = {
        analytics: SpendingAnalytic[]
    };

    const {
        analytics = $bindable([])
    }: Props = $props();
</script>

<aside>
    <div class="header">
        <h6 class="text-base font-medium">Categories</h6>
        <h6 class="text-base font-medium">Total spent</h6>
    </div>
    <ul class="space-y-5 overflow-auto h-full">
        {#each analytics as analytic (analytic.category.id)}
            <li class="flex items-start justify-between">
                <div class="w-full space-y-0.5">
                    <p>{analytic.category.title}</p>
                    <div style:--width={`${analytic.percentage*100}%`} class="bg-surface-primary w-(--width) h-2 rounded-xs"></div>
                </div>
                <p>{formatMoney(analytic.total.toString())}</p>
            </li>
        {/each}
    </ul>
</aside>

<style>
    aside {
        background: white;
        width: 100%;
        max-width: 400px;
        height: 100%;
        display: flex;
        flex-direction: column;
    }

    .header {
        display: flex;
        justify-content: space-between;
        border-bottom: 1px solid var(--color-neutral-50);
    }

    .header, ul {
        padding: 16px;
    }
</style>