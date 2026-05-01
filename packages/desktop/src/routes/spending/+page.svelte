<script lang="ts">
    import * as echarts from "echarts/core";
    import  {PieChart} from "echarts/charts";
    import {SVGRenderer} from "echarts/renderers";
    import { LabelLayout, UniversalTransition } from 'echarts/features';
    import {
      TitleComponent,
      TooltipComponent,
      GridComponent,
      DatasetComponent,
      TransformComponent
    } from 'echarts/components';

    // Only import certain parts to reduce bundle size
    echarts.use([
        LabelLayout,
        UniversalTransition,
        PieChart,
        SVGRenderer,
        TitleComponent,
        TooltipComponent,
        GridComponent,
        DatasetComponent,
        TransformComponent,
    ])
    let chart = echarts.init(document.getElementById("spending-pie-chart"));
</script>

<main>
    <div id="spending-pie-chart"></div>
</main>

<style>

    
</style>
