<script lang="ts" setup>
const props = defineProps<{
  text?: string
  type?: string
}>()
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

section.error {
  color: #ed6464;
}
</style>