<script lang="ts" setup>
import {onUpdated, ref} from "vue";

const props = defineProps<{
  logs?: string[]
}>();
const textAreaRef = ref<HTMLTextAreaElement>();

onUpdated(() => {
  if (!textAreaRef.value) {
    return;
  }
  textAreaRef.value.scrollTop = textAreaRef.value.scrollHeight;
})

</script>

<template>
  <textarea ref="textAreaRef" :value="props.logs?.join('\n\n')">
  </textarea>
</template>

<style scoped>
textarea {
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
}
</style>