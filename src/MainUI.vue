<script lang="ts" setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Container from "./components/Container.vue";
import InputField from "./components/InputField.vue";
import Button from "./components/Button.vue";
import LoggingArea from "./components/LoggingArea.vue";
import NetworkChart from "./components/NetworkChart.vue";

function onStartBtnClick(): boolean {
  console.log("On start button click");
  return true
}
</script>

<template>
  <div class="left_panel">
    <Container class="input_field_panel">
      <InputField hint="Register a user from ppaass website"
                  label="User token:" name="user_token"
                  place-holder="Enter the user token">
      </InputField>
      <InputField hint="Proxy addresses are separate with ;"
                  label="Proxy address:" name="proxy_address"
                  place-holder="Enter the proxy addresses">
      </InputField>
      <InputField hint="Listening port should between 1025~65536"
                  label="Listening port:" name="listening_port"
                  place-holder="Enter the listening port">
      </InputField>
    </Container>
    <Container class="button_panel">
      <Button label="Start agent" @on-click="onStartBtnClick()"></Button>
      <Button label="Stop agent"></Button>
    </Container>
    <Container class="network_panel">
      Network pannel<br/>
      Network pannel
    </Container>
    <Container class="status_panel">
      Status pannel
    </Container>
  </div>
  <div class="right_panel">
    <Container class="network_status">
      <NetworkChart/>
    </Container>
    <Container class="logging">
      <label>Logging:</label>
      <LoggingArea/>
    </Container>
  </div>
</template>

<style scoped>
.left_panel {
  width: 390px;
}

.loading_message {
  margin-left: auto;
  margin-right: auto;
  margin-top: 250px;
  font-size: 2em;
}

.input_field_panel {
  margin-top: 10px;
  display: flex;
  flex-direction: column;
  padding: 5px;
}

.button_panel {
  margin-top: 10px;
  display: flex;
  flex-direction: row;
  align-items: end;
  padding: 5px;
  justify-items: end;
  justify-content: end;
}

.network_panel {
  padding: 5px;
  margin-top: 10px;
  border: 1px solid #333333;
  color: #333333;
  font-size: 0.7em;
  font-weight: bold;
  display: flex;
  flex-direction: column;
}

.status_panel {
  padding: 5px;
  margin-top: 10px;
  border: 1px solid #333333;
  color: #aaaaaa;
  font-size: 0.7em;
  font-weight: bold;
  display: flex;
  flex-direction: column;
}

.status_panel span {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  overflow: hidden;
}

.right_panel {
  width: 390px;
  display: flex;
  flex-direction: column;
}

.right_panel .network_status {
  /* flex-grow: 1; */
  display: flex;
  flex-direction: column;
  margin-top: 10px;
}

.right_panel .logging {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
}

.right_panel .logging label {
  margin: 10px;
  flex-grow: 1;
  font-weight: bold;
}

.right_panel .logging textarea {
  /* margin: 5px; */
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
