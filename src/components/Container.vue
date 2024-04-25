<script lang="ts" setup>
const props = defineProps(['classes']);
</script>

<template>
  <div :class="props.classes" class="container">
    <slot></slot>
  </div>
</template>

<style scoped>
.container {
  padding: 10px;
  border: 1px solid #555555;
  border-radius: 5px;
  margin: 5px;
}
</style>