use std::collections::VecDeque;

use ppaass_agent_ui_model::model::{
    UiModelAgentServerConfiguration, UiModelNetworkDetail, UiModelStatusBarDetail,
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UiBackendCommand {
    StartAgentServer,
    StopAgentServer,
    LoadAgentServerConifugration,
}

impl UiBackendCommand {
    pub fn name(&self) -> &str {
        match self {
            UiBackendCommand::StartAgentServer => "start_agent_server",
            UiBackendCommand::StopAgentServer => "stop_agent_server",
            UiBackendCommand::LoadAgentServerConifugration => "load_agent_server_configuration",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiBackendCommandArgWrapper<T> {
    pub arg: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiBackendEventWrapper<T> {
    pub payload: T,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiStateMainPage {
    pub configuration: Option<UiModelAgentServerConfiguration>,
    pub network_detail: UiModelNetworkDetail,
    pub status_bar_detail: UiModelStatusBarDetail,
    pub network_chart_download_mb_per_second: VecDeque<f64>,
    pub network_chart_upload_mb_per_second: VecDeque<f64>,
}
