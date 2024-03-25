// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::OnceLock;

use clap::Parser;
use ppaass_agent::config::AgentConfig;
use ppaass_agent::error::AgentError;
use ppaass_agent::server::AgentServer;
use serde::{Deserialize, Serialize};
use tauri::async_runtime::spawn;

static AGENT_CONFIG: OnceLock<AgentConfig> = OnceLock::new();

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentUiInfo {
    user_token: String,
    proxy_address: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn start_vpn(info: AgentUiInfo) {
    println!("Receive agent ui info: {:?}", info);
    spawn(async {
        let config = AgentConfig::parse();
        match  AgentServer::new(config) {
            Ok(server) => server.start(),
            Err(e) => {
                println!("Fail to start agent server becuase of error: {e:?}")
            }
        }
    });
}

#[tauri::command]
fn stop_vpn() {
    println!("Going to stop vpn")
}

fn main() {
    AGENT_CONFIG.get_or_init(|| AgentConfig::parse());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_vpn, stop_vpn])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
