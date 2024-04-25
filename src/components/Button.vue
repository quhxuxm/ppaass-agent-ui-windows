<script lang="ts" setup>
const props = defineProps(["label", "classes"]);
const emits = defineEmits<{
  (eventName: "onclick", event: MouseEvent): void
}>();

function onClick(event: MouseEvent) {
  emits("onclick", event);
}

</script>

<template>
  <button :class="props.classes" @click="onClick">
    {{ props.label }}
  </button>
</template>

<style scoped>
button {
  font-size: 1em;
  margin: 10px;
  padding: 10px;
  font-weight: bold;
  border-radius: 8px;
  cursor: pointer;
  outline: none;
  box-shadow: none;
  border: 1px solid #0f0f0f;
  min-width: 100px;
  transition: border-color 0.25s;
  background-color: #ffffff;
  line-height: 1em;
}

button:hover {
  color: #333333;
  background-color: #eeeeee;
}

button:disabled {
  color: #999999;
  background-color: #eeeeee;
}
</style>