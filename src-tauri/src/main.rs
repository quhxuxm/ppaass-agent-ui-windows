// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentUiInfo {
    user_token: String,
    proxy_address: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn start_vpn(info: AgentUiInfo) {
    println!("Receive agent ui info: {:?}", info)
}

#[tauri::command]
fn stop_vpn() {
    println!("Going to stop vpn")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_vpn, stop_vpn])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
