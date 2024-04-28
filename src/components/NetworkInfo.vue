<script lang="ts" setup>
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";

const props = defineProps<{
  download_mb_amount?: number
  upload_mb_amount?: number
  download_mb_per_second?: number
  upload_mb_per_second?: number
}>();
</script>

<template>
  <div class="network_info">
    <section>
      <font-awesome-icon :icon="['fas','cloud-arrow-down']" class="icon download" size="lg"/>
      Total: {{ props.upload_mb_amount?.toFixed(2) }} MB; Avg: {{ props.upload_mb_per_second?.toFixed(2) }}
      MB/s
    </section>
    <section>
      <font-awesome-icon :icon="['fas','cloud-arrow-up']" class="icon upload" size="lg"/>
      Total: {{ props.download_mb_amount?.toFixed(2) }} MB; Avg: {{
        props.download_mb_per_second?.toFixed(2)
      }} MB/s
    </section>
  </div>
</template>

<style scoped>
.network_info {
  display: flex;
  flex-direction: column;
  margin: 5px;

}

.network_info section {
  font-size: 0.7em;
}

.network_info .icon {
  margin-right: 3px;
}

.network_info .icon.download {
  color: #3c81d5;
}

.network_info .icon.upload {
  color: #d5bc3c;
}
</style>