<script lang="ts" setup>
import {ref} from "vue";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";

const IP_PATTERN = /^(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\:([0-9]|[1-9]\d{1,3}|[1-5]\d{4}|6[0-5]{2}[0-3][0-5])$/;

const isError = ref<boolean>()

const props = defineProps<{
  id?: string,
  name?: string,
  label?: string,
  placeHolder?: string,
  hint?: string,
  disable?: boolean
  valueType?: "number" | "address" | "text",
  maxNumber?: number,
  minNumber?: number,
  icon?: string,
}>();

const value = defineModel<string>({
  set(val) {
    if (val.trim().length == 0) {
      isError.value = false;
      return val;
    }
    if (props.valueType == "number") {
      let parsedResult = Number(val);
      console.info("The value of the input is: ", val, ", the parsed result: ", parsedResult);
      if (isNaN(parsedResult)) {
        isError.value = true;
        return val;
      }
      if (props.maxNumber == undefined) {
        isError.value = false;
        return val
      }
      if (parsedResult > props.maxNumber) {
        isError.value = true;
        return val;
      }
      if (props.minNumber == undefined) {
        isError.value = false;
        return val
      }
      if (parsedResult < props.minNumber) {
        isError.value = true;
        return val;
      }
      isError.value = false;

      return val
    }
    if (props.valueType == "address") {
      let valParts = val.split(";");
      for (let part of valParts) {
        console.info("The part of the addresses: ", part);
        if (!IP_PATTERN.test(part)) {
          isError.value = true;
          return val;
        }
      }
      isError.value = false;
      return val;
    }
    isError.value = false;
    return val;
  },
});

</script>

<template>
  <div :class="{'disable':props.disable, 'error': isError}" class="input_field">
    <label v-if="props.label" :for="props.id">
      {{ props.label }}
    </label>
    <span class="field">
      <font-awesome-icon :icon="['fas',props.icon]" class="icon"/>
      <input :id="props.id"
             v-model="value"
             :disabled="props.disable"
             :name="props.name"
             :placeholder="props.placeHolder"
             type="text"/>
    </span>


    <span v-if="props.hint" class="hint">
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

.input_field.disable label {
  color: #999999;
}

.input_field span.field {
  position: relative;
  margin: 5px;
  display: flex;
  flex-direction: row;
}

.input_field span.field .icon {
  top: 11px;
  left: 8px;
  position: absolute;
  color: #555555;
}

.input_field span.field input {
  padding-top: 10px;
  padding-bottom: 10px;
  padding-right: 8px;
  padding-left: 33px;
  border: 1px solid #0f0f0f;
  border-radius: 5px;
  font-size: 1em;
  outline: none;
  box-shadow: none;
  flex-grow: 1;
}

.input_field.disable span.field input {
  color: #999999;
  border: 1px solid #999999;
}

.input_field.disable span.field .icon {
  color: #999999;
}

.input_field.error span.field .icon {
  color: #ed6464;
}

.input_field.error span.field input {
  color: #ed6464;
  border: 1px solid #ed6464;
}


.input_field span.hint {
  font-size: 0.9em;
  color: #555555;
  margin: 5px;
  padding: 5px;
}

.input_field.disable span.hint {
  color: #999999;
}

.input_field.error span.hint {
  color: #ed6464;
}

</style>