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

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentUiInfo {
    user_token: String,
    proxy_address: String,
    listening_port: String,
}

#[derive(Default)]
pub struct AgentUiState {
    agent_server_guard: Mutex<Option<AgentServerGuard>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn start_vpn(
    info: AgentUiInfo,
    state: State<'_, AgentUiState>,
) -> Result<(), PpaassAgentUiError> {
    println!("Receive agent ui info: {:?}", info);
    let mut config = AgentConfig::parse();
    let proxy_addresses = info
        .proxy_address
        .split(";")
        .into_iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>();
    let listening_port = info.listening_port.parse::<u16>().map_err(|e| {
        PpaassAgentUiError::Other(format!(
            "Fail to parse listening port because of error: {e:?}"
        ))
    })?;
    // config.set_user_token(info.user_token);
    // config.set_proxy_addresses(proxy_addresses);
    config.set_port(listening_port);
    let agent_server =
        AgentServer::new(config).map_err(|e| PpaassAgentUiError::Agent(format!("{e:?}")))?;
    let server_guard = agent_server.start();
    let mut agent_server_guard_state = state
        .agent_server_guard
        .lock()
        .map_err(|e| PpaassAgentUiError::Other(format!("{e:?}")))?;
    *agent_server_guard_state = Some(server_guard);
    Ok(())
}

#[tauri::command]
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
    tauri::Builder::default()
        .manage(AgentUiState::default())
        .invoke_handler(tauri::generate_handler![start_vpn, stop_vpn])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
