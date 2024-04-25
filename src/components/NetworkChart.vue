<script lang="ts" setup>

</script>

<template>
  <div class="network_chart_container">
    <label>Network status</label>
    <div class="network_chart"></div>
  </div>
</template>

<style scoped>
div.network_chart_container {
  margin: 10px;
  flex-grow: 1;
  font-weight: bold;
  display: flex;
  flex-direction: column;
}

div.network_chart_container label {
  margin-bottom: 10px;
}

div.network_chart {
  width: 100%;
  height: 250px;
  border: 1px solid
}
</style>