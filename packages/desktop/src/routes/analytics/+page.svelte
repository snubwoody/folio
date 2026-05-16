<script lang="ts">
    import * as echarts from "echarts/core";
    import  { PieChart, type PieSeriesOption } from "echarts/charts";
    import { SVGRenderer } from "echarts/renderers";
    import { LabelLayout, UniversalTransition } from "echarts/features";
    import {
        TitleComponent,
        TooltipComponent,
        GridComponent,
        DatasetComponent,
        TransformComponent
    } from "echarts/components";
    import {
        AriaComponent,
        LegendComponent,
        type TitleComponentOption,
        type TooltipComponentOption,
        type GridComponentOption,
        type LegendComponentOption,
        type DatasetComponentOption,
        type AriaComponentOption
    } from "echarts/components";
    import type { ComposeOption } from "echarts/core";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { getLocalTimeZone, today } from "@internationalized/date";
    import { categoryStore } from "$lib/stores/categories.svelte";
    import { spendingAnalytics } from "$lib/analytics";
    import CategorySidebar from "$components/analytics/CategorySidebar.svelte";
    import { IconButton } from "$components/button";
    import { ChevronLeft,ChevronRight } from "@lucide/svelte";

    type ECOption = ComposeOption<
        | PieSeriesOption
        | LegendComponentOption
        | TitleComponentOption
        | TooltipComponentOption
        | GridComponentOption
        | DatasetComponentOption
        | AriaComponentOption
    >;

    // Only import certain parts to reduce bundle size
    echarts.use([
        LabelLayout,
        UniversalTransition,
        PieChart,
        SVGRenderer,
        LegendComponent,
        TitleComponent,
        TooltipComponent,
        GridComponent,
        AriaComponent,
        DatasetComponent,
        TransformComponent
    ]);

    let month = $state(today(getLocalTimeZone()));
    let analytics = $derived.by(() => spendingAnalytics(transactionStore.transactions,categoryStore.categoryMap,{ month }));

    // TODO: disable start animation
    // TODO: use filled pie chart

    let seriesData = $derived(
        analytics.map(a => {
            const itemStyle = {
                borderRadius: 12,
                color: a.color
            };
            return { name: a.category.title, value: a.total, itemStyle };
        })
    );

    const option: ECOption = $derived({
        aria: {
            enabled: true
        },
        legend: {
            show: false
        },
        series: [
            {
                type: "pie",
                // radius: ["50%", "70%"],
                padAngle: 0.5,
                avoidLabelOverlap: false,
                labelLine: {
                    show: true,
                    position: "center"
                },
                // emphasis: {
                //     label: {
                //         show: true,
                //         fontSize: 16,
                //         fontWeight: 'bold'
                //     }
                // },
                data: seriesData
            }
        ]
    });

    // TODO: show category on hover
    // TODO: check empty chart

    const formatter = new Intl.DateTimeFormat("en-GB",{
        month: "short",
        year:"numeric"
    });

    $effect(() => {
        // eslint-disable-next-line no-undef
        let chart = echarts.init(document.getElementById("spending-pie-chart"));
        chart.setOption(option);
    });
</script>

<main>
    <section class="overflow-y-auto space-y-2">
        <header class="flex items-center justify-between">
            <h6>Spending breakdown</h6>
            <div class="flex items-center gap-0.5">
                <IconButton variant="ghost" onclick={() => month = month.subtract({ months: 1 })}>
                    <ChevronLeft/>
                </IconButton>
                <p>{formatter.format(month.toDate(getLocalTimeZone()))}</p>
                <IconButton variant="ghost" onclick={() => month = month.add({ months: 1 })}>
                    <ChevronRight/>
                </IconButton>
            </div>
        </header>
        <div class="chart-wrapper">
            <div id="spending-pie-chart"></div>
        </div>
    </section>
    <CategorySidebar bind:analytics/>
</main>

<style>
    main{
        display: flex;
        width: 100%;
        height: 100%;
        background: var(--color-neutral-25);
    }

    section {
        width: 100%;
        padding: 20px;
    }

    .chart-wrapper{
        display: flex;
        justify-content: center;
        width: 100%;
    }

    #spending-pie-chart {
        width: 100%;
        max-width: 850px;
        aspect-ratio: 1/1;
    }
</style>
