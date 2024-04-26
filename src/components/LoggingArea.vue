<script lang="ts" setup>
import {onUpdated, ref} from "vue";

const props = defineProps<{
  logs?: {
    logType: "info" | "warn" | "error",
    content: string,
    isAgentServer: boolean
  }[]
}>();
const logContainerRef = ref<HTMLTextAreaElement>();

onUpdated(() => {
  if (!logContainerRef.value) {
    return;
  }
  logContainerRef.value.scrollTop = logContainerRef.value.scrollHeight;
})

</script>

<template>
  <ul ref="logContainerRef">
    <li v-for="item in props.logs" :class="item.logType">
      <span :class="{'agentServer': item.isAgentServer}">{{ item.content }}</span>
    </li>
  </ul>
</template>

<style scoped>
ul {
  flex-grow: 40;
  border: 1px solid #555555;
  outline: none;
  box-shadow: none;
  resize: none;
  padding: 10px;
  font-size: 0.7em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  line-break: anywhere;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  height: 225px;
  text-overflow: ellipsis;
}

li {
  margin-bottom: 5px;
  display: block;
  width: 100%;
}

li.info {
  color: #555555;
}

li.error {
  color: #ed6464;
  font-weight: bold;
}

li.warn {
  color: #d5bc3c;
  font-weight: bold;
}

li:last-child {
  border-bottom: none;
}

li span {
  width: 100%;
  display: inline-block;
  align-content: center;
  padding: 5px;
  text-overflow: ellipsis;
}

li.info span.agentServer {
  color: #3c81d5;
  font-weight: bold;
}

</style>