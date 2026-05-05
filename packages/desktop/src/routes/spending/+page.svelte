<script lang="ts">
    import * as echarts from "echarts/core";
    import  { PieChart,type PieSeriesOption } from "echarts/charts";
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
        // The component option types are defined with the ComponentOption suffix
        type TitleComponentOption,
        type TooltipComponentOption,
        type GridComponentOption,
        LegendComponent,
        type LegendComponentOption,
        type DatasetComponentOption
    } from "echarts/components";
    import type {
        ComposeOption
    } from "echarts/core";
    import { transactionStore } from "$lib/stores/transaction.svelte";
    import { getLocalTimeZone, today } from "@internationalized/date";
    import { categoryStore } from "$lib/stores/categories.svelte";
    import { spendingAnalytics } from "$lib/analytics";
    import { onMount } from "svelte";

    // Create an Option type with only the required components and charts via ComposeOption
    type ECOption = ComposeOption<
      | PieSeriesOption
      | LegendComponentOption
      | TitleComponentOption
      | TooltipComponentOption
      | GridComponentOption
      | DatasetComponentOption
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
        DatasetComponent,
        TransformComponent
    ]);

    let analytics = $derived.by(() => spendingAnalytics(transactionStore.transactions,categoryStore.categoryMap,{ month: today(getLocalTimeZone()) }));

    let legendData = $derived(analytics.map(a => a.category.title));
    let seriesData = $derived(
        analytics.map(a => {
            return { name:a.category.title,value: a.total };
        })
    );

    const option: ECOption = $derived({
        legend: {
            orient: "vertical",
            x: "left",
            data: legendData
        },
        series: [
            {
                type: "pie",
                radius: ["50%", "70%"],
                avoidLabelOverlap: false,
                labelLine: {
                    show: true
                },
                data: seriesData
            }
        ]
    });

    // TODO: invert colours
    onMount(() => {
        // eslint-disable-next-line no-undef
        let chart = echarts.init(document.getElementById("spending-pie-chart"));
        chart.setOption(option);
    });
</script>

<main>
    <div class="chart-wrapper">
        <div id="spending-pie-chart"></div>
    </div>
</main>

<style>
    main{
        display: flex;
        width: 100%;
        height: 100%;
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

        /* height:  500px; */
    }

</style>
