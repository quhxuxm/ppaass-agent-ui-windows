<script lang="ts" setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Container from "./components/Container.vue";
import Button from "./components/Button.vue";
import LoggingArea from "./components/LoggingArea.vue";
import NetworkChart from "./components/NetworkChart.vue";
import NetworkInfo from "./components/NetworkInfo.vue";
import SystemStatus from "./components/SystemStatus.vue";
import {invoke} from "@tauri-apps/api/tauri";
import {onMounted, ref} from "vue";
import {AgentServerConfiguration} from "./vo/AgentServerConfiguration"
import InputField from "./components/InputField.vue";

const userToken = ref<string>();
const proxyAddresses = ref<string>();
const port = ref<string>()

onMounted(() => {
  invoke<AgentServerConfiguration>("load_agent_server_configuration").then(value => {
    console.info("Success to load agent server configuration:", value);
    userToken.value = value.userToken;
    proxyAddresses.value = value.proxyAddresses?.join(";");
    port.value = value.port?.toString();
  }).catch(error => {
    console.error("Fail to load agent server configuration:", error);
  });
});

function onStartBtnClick() {
  let agentConfiguration = new AgentServerConfiguration(userToken.value, proxyAddresses.value?.split(";"), parseInt(port.value ? port.value : "0"))
  console.log("On start button click, current agent configuration will be: ", agentConfiguration);
  invoke("start_agent_server", {
    arg: agentConfiguration
  });
}

function onStopBtnClick(event: MouseEvent) {
  console.log("On stop button click, receive event: ", event);
  invoke("stop_agent_server");
}
</script>

<template>
  <div class="left_panel">
    <Container class="input_field_panel">
      <InputField
          v-model="userToken"
          hint="Register a user from ppaass website"
          label="User token:"
          name="user_token"
          place-holder="Enter the user token"
          value-type="text">
      </InputField>
      <InputField
          v-model="proxyAddresses"
          hint="Proxy addresses are separate with ;"
          label="Proxy address:"
          name="proxy_address"
          place-holder="Enter the proxy addresses"
          value-type="address">
      </InputField>
      <InputField
          v-model="port"
          :max-number="65535"
          :min-number="1025"
          hint="Listening port should between 1025~65535"
          label="Listening port:"
          name="listening_port" place-holder="Enter the listening port" value-type="number">
      </InputField>
    </Container>
    <Container class="button_panel">
      <Button label="Start" @onclick="onStartBtnClick"></Button>
      <Button label="Stop" @onclick="onStopBtnClick"></Button>
    </Container>
    <Container class="network_panel">
      <NetworkInfo :download_mb_amount="0" :download_mb_per_second="0" :upload_mb_amount="0" :upload_mb_per_second="0"/>
    </Container>
    <Container class="status_panel">
      <SystemStatus text="Agent server started success, listning on port: 10080." type="error"/>
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
