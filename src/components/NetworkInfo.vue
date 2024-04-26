<script lang="ts" setup>
const props = defineProps<{
  download_mb_amount?: number
  upload_mb_amount?: number
  download_mb_per_second?: number
  upload_mb_per_second?: number
}>();
</script>

<template>
  <div class="network_info">
    <section>↑↑↑ Total: {{ props.upload_mb_amount?.toFixed(2) }} MB; Avg: {{ props.upload_mb_per_second?.toFixed(2) }}
      MB/S
    </section>
    <section>↓↓↓ Total: {{ props.download_mb_amount?.toFixed(2) }} MB; Avg: {{
        props.download_mb_per_second?.toFixed(2)
      }} MB/S
    </section>
  </div>
</template>

<style scoped>
.network_info {
  display: flex;
  flex-direction: column;
  margin: 5px;
}
</style>