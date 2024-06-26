<script lang="ts" setup>
import {onBeforeUpdate, onMounted, onUnmounted, ref} from "vue";
import {EChartOption, ECharts, init} from "echarts";

const props = defineProps<{
  downloadMbPerSecondArray: number[],
  uploadMbPerSecondArray: number[]
}>();


const chartContentRef = ref<HTMLDivElement>();

let chartInstance: ECharts;
onMounted(() => {
  const option: EChartOption = {
    title: {
      text: "Network status"
    },

    axisPointer: {
      show: true,
      snap: true,
      lineStyle: {
        color: "#909090"
      }
    },
    grid: {
      left: 10,
      right: 10,
      bottom: 10,
      top: 60,
    },
    xAxis: {
      data: [],
      show: true,
    },
    yAxis: {
      type: 'value',
      interval: Number.MAX_VALUE,
      offset: -250,
      axisLabel: {
        show: true,
        showMinLabel: false,
        showMaxLabel: true,
        formatter: function (value: any) {
          let valueAsString = parseFloat(value).toFixed(2);
          return `${valueAsString} MB/s`
        }
      }
    },
    series: [
      {
        name: "downloadMbPerSecond",
        type: "line",
        data: props.downloadMbPerSecondArray,
        lineStyle: {
          color: "#3c81d5",
          width: 1
        },
        areaStyle: {
          color: "#EEEEEE"
        },
        itemStyle: {
          color: "#3c81d5",
          borderWidth: 1,
          areaColor: "#81a8d6",
        },
      },
      {
        name: "uploadMbPerSecond",
        type: "line",
        data: props.uploadMbPerSecondArray,
        lineStyle: {
          color: "#d5bc3c",
          width: 1
        },
        itemStyle: {
          color: "#d5bc3c",
          borderWidth: 1,
          areaColor: "#d6c881",
        },
        areaStyle: {
          color: "#EEEEEE"
        }
      }
    ]
  };
  chartInstance = init(chartContentRef.value);
  chartInstance.setOption(option);
});

onBeforeUpdate(() => {
  chartInstance.setOption({
    series: [
      {
        name: "downloadMbPerSecond",
        type: "line",
        data: props.downloadMbPerSecondArray,
      },
      {
        name: "uploadMbPerSecond",
        type: "line",
        data: props.uploadMbPerSecondArray,
      }
    ]
  });
})

onUnmounted(() => {
  chartInstance.dispose();
})

</script>

<template>
  <div ref="chartContentRef" class="network_chart_container">
  </div>
</template>

<style scoped>
div.network_chart_container {
  flex-grow: 1;
  font-weight: bold;
  display: flex;
  flex-direction: column;
  height: 250px;
}

div.network_chart_container label {
  margin-bottom: 10px;
}

.chart {
  width: 100%;
  height: 250px;
  border: 1px solid
}
</style>