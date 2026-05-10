<script lang="ts">
    import {type SpendingAnalytic, spendingAnalytics} from "$lib/analytics";
    import {transactionStore} from "$lib/stores/transaction.svelte";
    import {categoryStore} from "$lib/stores/categories.svelte";
    import {getLocalTimeZone,today} from "@internationalized/date";
    import {formatMoney} from "$lib/utils/money";

    type Props = {
        analytics: SpendingAnalytic[]
    }

    const {
        analytics = $bindable([])
    } = $props()
</script>

<aside>
    <div class="header">
        <h6 class="text-base font-medium">Categories</h6>
        <h6 class="text-base font-medium">Total spending</h6>
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
        width: 100%;
        max-width: 300px;
        height: 100%;
        /*padding: 20px;*/
        display: flex;
        flex-direction: column;
        gap: 24px;
    }

    .header {
        display: flex;
        justify-content: space-between;
        border-bottom: 1px solid var(--color-neutral-50);
    }

    .header,ul{
        padding: 16px 8px;
    }
</style>