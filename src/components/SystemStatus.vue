<script lang="ts" setup>


const props = defineProps<{
  text?: string,
  type?: "info" | "error" | "warn"
}>();


</script>

<template>
  <section :class="props.type">{{ props.text }}</section>
</template>

<style scoped>
section {
  padding: 3px;
}

section.info {
  color: #3c81d5;
}

section.warn {
  color: #d5bc3c;
}

section.error {
  color: #ed6464;
}
</style>