<script lang="ts">
    import * as echarts from "echarts/core";
    import  {PieChart,type PieSeriesOption} from "echarts/charts";
    import {SVGRenderer} from "echarts/renderers";
    import { LabelLayout, UniversalTransition } from 'echarts/features';
    import {
      TitleComponent,
      TooltipComponent,
      GridComponent,
      DatasetComponent,
      TransformComponent
    } from 'echarts/components';
    import {
      // The component option types are defined with the ComponentOption suffix
      type TitleComponentOption, 
      type TooltipComponentOption,
      type GridComponentOption,
      LegendComponent,
      type LegendComponentOption,
      type DatasetComponentOption
    } from 'echarts/components';

    import type { 
      ComposeOption, 
    } from 'echarts/core';
    
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
        TransformComponent,
    ])

    const option: ECOption = {
      legend: {
        orient: 'vertical',
        x: 'left',
        data: ['A', 'B', 'C', 'D', 'E']
      },
      series: [
        {
          type: 'pie',
          radius: ['50%', '70%'],
          avoidLabelOverlap: false,
          label: {
            show: false,
            position: 'center'
          },
          labelLine: {
            show: false
          },
          emphasis: {
            label: {
              show: true,
              fontSize: '30',
              fontWeight: 'bold'
            }
          },
          data: [
            { value: 335, name: 'A' },
            { value: 310, name: 'B' },
            { value: 234, name: 'C' },
            { value: 135, name: 'D' },
            { value: 1548, name: 'E' }
          ]
        }
      ]
    };

    $effect(()=>{
        let chart = echarts.init(document.getElementById("spending-pie-chart"));
        chart.setOption(option)
    });
</script>

<main>
    hi
    <div id="spending-pie-chart"></div>
</main>

<style>
    #spending-pie-chart {
        width: 500px;
        height:  500px;
    }
    
</style>
