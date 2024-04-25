<script lang="ts" setup>
const props = defineProps(["id", "name", "label", "placeHolder", "hint"]);
</script>

<template>
  <div class="input_field">
    <label v-if="props.label" :for="props.id">
      {{ props.label }}
    </label>
    <input :id="props.id" :name="props.name" :placeholder="props.placeHolder" type="text"/>
    <span v-if="props.hint">
      {{ props.hint }}
    </span>
  </div>

</template>

<style scoped>
.input_field {
  display: flex;
  flex-direction: column;
}

.input_field label {
  font-size: 1.1em;
  margin: 5px;
  padding: 5px;
  font-weight: bold;
}

.input_field input {
  margin: 5px;
  padding: 8px;
  border: 1px solid #0f0f0f;
  border-radius: 5px;
  font-size: 1em;
  outline: none;
  box-shadow: none;
}

.input_field span {
  font-size: 0.9em;
  color: #555555;
  margin: 5px;
  padding: 5px;
}

.input_field span.error {
  color: #f27272;
}

.input_field input.error {
  border-color: #f27272;
  color: #f27272;
}
</style>