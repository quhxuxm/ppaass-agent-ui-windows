// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use clap::Parser;
use ppaass_agent::config::AgentConfig;
use ppaass_agent::server::{AgentServer, AgentServerGuard};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::error::PpaassAgentUiError;

mod error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentUiInfo {
    #[serde(rename = "user_token")]
    user_token: String,
    #[serde(rename = "proxy_address")]
    proxy_address: String,
    #[serde(rename = "listening_port")]
    listening_port: String,
}

pub struct AgentUiState {
    current_ui_state: AgentUiInfo,
    agent_server_guard: Mutex<Option<AgentServerGuard>>,
}

#[tauri::command(rename_all = "snake_case")]
fn load_ui_info(state: State<'_, AgentUiState>) -> String {
    serde_json::to_string(&state.current_ui_state).unwrap()
}

#[tauri::command(rename_all = "snake_case")]
async fn start_vpn(
    info: AgentUiInfo,
    state: State<'_, AgentUiState>,
) -> Result<(), PpaassAgentUiError> {
    println!("Receive agent ui info: {:?}", info);

    let proxy_addresses = info
        .proxy_address
        .split(';')
        .map(|item| item.to_string())
        .collect::<Vec<String>>();
    let listening_port = info.listening_port.parse::<u16>().map_err(|e| {
        PpaassAgentUiError::Other(format!(
            "Fail to parse listening port because of error: {e:?}"
        ))
    })?;
    let mut current_server_config = AgentConfig::parse();
    current_server_config.set_user_token(info.user_token);
    current_server_config.set_proxy_addresses(proxy_addresses);
    current_server_config.set_port(listening_port);
    let agent_server = AgentServer::new(current_server_config)
        .map_err(|e| PpaassAgentUiError::Agent(format!("{e:?}")))?;
    let server_guard = agent_server.start();
    let mut agent_server_guard_state = state
        .agent_server_guard
        .lock()
        .map_err(|e| PpaassAgentUiError::Other(format!("{e:?}")))?;
    *agent_server_guard_state = Some(server_guard);
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
async fn stop_vpn(state: State<'_, AgentUiState>) -> Result<(), PpaassAgentUiError> {
    println!("Going to stop vpn");
    let mut agent_server_guard_state = state
        .agent_server_guard
        .lock()
        .map_err(|e| PpaassAgentUiError::Other(format!("{e:?}")))?;
    let _ = agent_server_guard_state.take();
    Ok(())
}

fn main() {
    let current_server_config = AgentConfig::parse();
    let initial_state = AgentUiState {
        current_ui_state: AgentUiInfo {
            user_token: current_server_config.user_token().to_string(),
            proxy_address: current_server_config.proxy_addresses().clone().join(";"),
            listening_port: current_server_config.port().to_string(),
        },
        agent_server_guard: Mutex::new(None),
    };

    tauri::Builder::default()
        .manage(initial_state)
        .invoke_handler(tauri::generate_handler![start_vpn, stop_vpn, load_ui_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
