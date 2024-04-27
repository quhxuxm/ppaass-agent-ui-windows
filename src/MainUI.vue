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
import {onMounted, onUnmounted, ref} from "vue";
import {AgentServerConfiguration} from "./vo/AgentServerConfiguration"
import InputField from "./components/InputField.vue";
import {AgentServerEvent} from "./vo/AgentServerEvent.ts";
import {listen, UnlistenFn} from "@tauri-apps/api/event";
import {AgentServerEventType} from "./vo/AgentServerEventType.ts";
import {NetworkState} from "./vo/NetworkState.ts";

const userToken = ref<string>();
const proxyAddresses = ref<string>();
const port = ref<string>()
const systemStatusText = ref<string>("Loading ...");
const systemStatusType = ref<"info" | "error" | "warn">();

const downloadMbAmount = ref<number>(0);
const downloadMbPerSecond = ref<number>(0);
const uploadMbAmount = ref<number>(0);
const uploadMbPerSecond = ref<number>(0);

const loggingRecordsRef = ref<{
  logType: "info" | "warn" | "error",
  content: string,
  isAgentServer: boolean
}[]>([]);

const downloadMbPerSecondArrayRef = ref<number[]>([]);
const uploadMbPerSecondArrayRef = ref<number[]>([]);

const started = ref<boolean>(false);


onMounted(() => {
  invoke<AgentServerConfiguration>("load_agent_server_configuration").then(value => {
    console.info("Success to load agent server configuration:", value);
    userToken.value = value.userToken;
    proxyAddresses.value = value.proxyAddresses?.join(";");
    port.value = value.port?.toString();
    systemStatusText.value = "Ready to start agent server.";
  }).catch(error => {
    console.error("Fail to load agent server configuration:", error);
  });
});

function pushLoggingRecords(loggingRecord: {
  logType: "info" | "error" | "warn",
  content: string,
  isAgentServer: boolean
}) {
  loggingRecordsRef.value.push({
    logType: loggingRecord.logType,
    content: loggingRecord.content,
    isAgentServer: loggingRecord.isAgentServer
  });
  loggingRecordsRef.value = loggingRecordsRef.value.slice(0, loggingRecordsRef.value.length);
  if (loggingRecordsRef.value.length > 100) {
    loggingRecordsRef.value = loggingRecordsRef.value.slice(1, loggingRecordsRef.value.length);
  }
}

let unListenAgentServerEvent: UnlistenFn;
listen<AgentServerEvent>("__AGENT_SERVER_EVENT__", (event) => {
  console.info("Receive server event: ", event.payload)
  if (event.payload.eventType == AgentServerEventType.StartSuccess) {
    systemStatusText.value = event.payload.content;
    systemStatusType.value = "info";
    started.value = true;
    pushLoggingRecords({
      logType: "info",
      content: event.payload.content,
      isAgentServer: true
    })
    return;
  }
  if (event.payload.eventType == AgentServerEventType.StartFail) {
    systemStatusText.value = event.payload.content;
    systemStatusType.value = "error";
    started.value = false;
    pushLoggingRecords({
      logType: "error",
      content: event.payload.content,
      isAgentServer: true
    })
    return;
  }
  if (event.payload.eventType == AgentServerEventType.StopSuccess) {
    systemStatusText.value = event.payload.content;
    systemStatusType.value = "info";
    started.value = false;
    pushLoggingRecords({
      logType: "info",
      content: event.payload.content,
      isAgentServer: true
    })
    return;
  }
  if (event.payload.eventType == AgentServerEventType.StopFail) {
    systemStatusText.value = event.payload.content;
    systemStatusType.value = "error";
    started.value = false;
    pushLoggingRecords({
      logType: "error",
      content: event.payload.content,
      isAgentServer: true
    })
    return;
  }
  if (event.payload.eventType == AgentServerEventType.Logging) {
    pushLoggingRecords({
      logType: "info",
      content: event.payload.content,
      isAgentServer: false
    })
    return;
  }
  if (event.payload.eventType == AgentServerEventType.NetworkState) {
    let networkState = JSON.parse(event.payload.content) as NetworkState;
    downloadMbAmount.value = networkState.downloadMbAmount;
    downloadMbPerSecond.value = networkState.downloadMbPerSecond;
    uploadMbAmount.value = networkState.uploadMbAmount;
    uploadMbPerSecond.value = networkState.uploadMbPerSecond;
    let downloadPerSecondArray = downloadMbPerSecondArrayRef.value.slice(0, downloadMbPerSecondArrayRef.value.length);
    downloadPerSecondArray.push(networkState.downloadMbPerSecond);
    if (downloadPerSecondArray.length > 50) {
      downloadPerSecondArray = downloadPerSecondArray.slice(downloadPerSecondArray.length - 50, downloadPerSecondArray.length);
    }
    downloadMbPerSecondArrayRef.value = downloadPerSecondArray;

    let uploadPerSecondArray = uploadMbPerSecondArrayRef.value.slice(0, uploadMbPerSecondArrayRef.value.length);
    uploadPerSecondArray.push(networkState.uploadMbPerSecond);
    if (uploadPerSecondArray.length > 50) {
      uploadPerSecondArray = uploadPerSecondArray.slice(uploadPerSecondArray.length - 50, uploadPerSecondArray.length);
    }
    uploadMbPerSecondArrayRef.value = uploadPerSecondArray;
    return;
  }
}).then((unListen) => {
  unListenAgentServerEvent = unListen;
});


onUnmounted(() => {
  if (unListenAgentServerEvent) {
    unListenAgentServerEvent();
  }

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
          :disable="started"
          hint="Register a user from ppaass website"
          icon="user"
          label="User token:"
          name="user_token"
          place-holder="Enter the user token" value-type="text">
      </InputField>
      <InputField
          v-model="proxyAddresses"
          :disable="started"
          hint="Proxy addresses are separate with ;"
          icon="network-wired"
          label="Proxy address:"
          name="proxy_address"
          place-holder="Enter the proxy addresses" value-type="address">
      </InputField>
      <InputField
          v-model="port"
          :disable="started"
          :max-number="65535"
          :min-number="1025"
          hint="Listening port should between 1025~65535"
          icon="ethernet"
          label="Listening port:" name="listening_port" place-holder="Enter the listening port" value-type="number">
      </InputField>
    </Container>
    <Container class="button_panel">
      <Button :disable="started" icon="play" label="Start" @onclick="onStartBtnClick"></Button>
      <Button :disable="!started" icon="power-off" label="Stop" @onclick="onStopBtnClick"></Button>
    </Container>
    <Container class="network_panel">
      <NetworkInfo :download_mb_amount="downloadMbAmount" :download_mb_per_second="downloadMbPerSecond"
                   :upload_mb_amount="uploadMbAmount" :upload_mb_per_second="uploadMbPerSecond"/>
    </Container>
    <Container class="status_panel">
      <SystemStatus :text="systemStatusText" :type="systemStatusType"/>
    </Container>
  </div>
  <div class="right_panel">
    <Container class="network_status">
      <NetworkChart :download-mb-per-second-array="downloadMbPerSecondArrayRef"
                    :upload-mb-per-second-array="uploadMbPerSecondArrayRef"/>
    </Container>
    <Container class="logging">
      <label>Logging:</label>
      <LoggingArea :logs="loggingRecordsRef"/>
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
