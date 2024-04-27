<script lang="ts" setup>
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";

const props = defineProps(["label", "classes", "disable", "icon"]);
const emits = defineEmits<{
  (eventName: "onclick", event: MouseEvent): void
}>();

function onClick(event: MouseEvent) {
  emits("onclick", event);
}

</script>

<template>
  <span>
    <font-awesome-icon :class="{'disabled':props.disable}" :icon="['fas',props.icon]" class="icon"/>
    <button :class="props.classes" :disabled="props.disable" @click="onClick">
      {{ props.label }}
    </button>
  </span>

</template>

<style scoped>
span {
  position: relative;
  display: inline-block;
}

span .icon {
  position: absolute;
  top: 20px;
  left: 25px;
  cursor: pointer;
  color: #555555;
}

button {
  font-size: 1em;
  margin: 10px;
  padding: 10px;
  padding-left: 30px;
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

span .icon:hover {
  color: #333333;
}

button:disabled {
  color: #999999;
  background-color: #eeeeee;
}

span .icon.disabled {
  color: #999999;
}
</style>