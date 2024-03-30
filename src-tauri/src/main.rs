// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use clap::Parser;
use ppaass_agent::config::AgentConfig;
use ppaass_agent::server::{AgentServer, AgentServerGuard};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{
    CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, WindowEvent,
};

use crate::error::PpaassAgentUiError;

mod error;

const SYSTEM_TRAY_MENU_ITEM_START_AGENT: &str = "SYSTEM_TRAY_MENU_ITEM_START_AGENT";
const SYSTEM_TRAY_MENU_ITEM_STOP_AGENT: &str = "SYSTEM_TRAY_MENU_ITEM_STOP_AGENT";
const SYSTEM_TRAY_MENU_ITEM_EXIT: &str = "SYSTEM_TRAY_MENU_ITEM_EXIT";
const MAIN_WINDOW_LABEL: &str = "main";

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
fn start_vpn(info: AgentUiInfo, state: State<'_, AgentUiState>) -> Result<(), PpaassAgentUiError> {
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
    let agent_server_guard = agent_server.start();
    let mut agent_server_guard_state = state.agent_server_guard.lock().unwrap();
    *agent_server_guard_state = Some(agent_server_guard);
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
fn stop_vpn(state: State<'_, AgentUiState>) -> Result<(), PpaassAgentUiError> {
    println!("Going to stop vpn");
    let mut agent_server_guard_state = state.agent_server_guard.lock().unwrap();
    let _ = agent_server_guard_state.take();
    Ok(())
}

fn main() {
    let current_server_config = AgentConfig::parse();
    let _log_guard =
        ppaass_agent::log::init_log(&current_server_config).expect("Fail to initialize log");
    let initial_state = AgentUiState {
        current_ui_state: AgentUiInfo {
            user_token: current_server_config.user_token().to_string(),
            proxy_address: current_server_config.proxy_addresses().clone().join(";"),
            listening_port: current_server_config.port().to_string(),
        },
        agent_server_guard: Mutex::new(None),
    };

    let start_menu_item =
        CustomMenuItem::new(SYSTEM_TRAY_MENU_ITEM_START_AGENT.to_string(), "Start agent");
    let stop_menu_item =
        CustomMenuItem::new(SYSTEM_TRAY_MENU_ITEM_STOP_AGENT.to_string(), "Stop agent");
    let exit_menu_item = CustomMenuItem::new(SYSTEM_TRAY_MENU_ITEM_EXIT.to_string(), "Exit");
    let system_tray_menu = SystemTrayMenu::new()
        .add_item(start_menu_item)
        .add_item(stop_menu_item)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(exit_menu_item);
    let system_tray = SystemTray::new().with_menu(system_tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick {
                id: menu_item_id, ..
            } => match menu_item_id.as_str() {
                SYSTEM_TRAY_MENU_ITEM_EXIT => {
                    std::process::exit(0);
                }
                SYSTEM_TRAY_MENU_ITEM_STOP_AGENT => {
                    if let Some(window) = app.get_window(MAIN_WINDOW_LABEL) {
                        let state = window.state::<AgentUiState>();
                        let mut agent_server_guard_state = state.agent_server_guard.lock().unwrap();
                        let _ = agent_server_guard_state.take();
                    }
                }
                SYSTEM_TRAY_MENU_ITEM_START_AGENT => {
                    if let Some(window) = app.get_window(MAIN_WINDOW_LABEL) {
                        let state = window.state::<AgentUiState>();
                        let proxy_addresses = state
                            .current_ui_state
                            .proxy_address
                            .split(';')
                            .map(|item| item.to_string())
                            .collect::<Vec<String>>();
                        let listening_port = state
                            .current_ui_state
                            .listening_port
                            .parse::<u16>()
                            .unwrap();
                        let mut current_server_config = AgentConfig::parse();
                        current_server_config
                            .set_user_token(state.current_ui_state.user_token.clone());
                        current_server_config.set_proxy_addresses(proxy_addresses);
                        current_server_config.set_port(listening_port);
                        let agent_server = AgentServer::new(current_server_config).unwrap();
                        let agent_server_guard = agent_server.start();
                        let mut agent_server_guard_state = state.agent_server_guard.lock().unwrap();
                        *agent_server_guard_state = Some(agent_server_guard);
                    }
                }
                _ => {}
            },
            SystemTrayEvent::LeftClick { .. } => {
                if let Some(window) = app.get_window("main") {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            _ => {}
        })
        .manage(initial_state)
        .on_window_event(|global_event| {
            if let WindowEvent::Resized(_) = global_event.event() {
                if global_event.window().is_minimized().unwrap() {
                    global_event.window().hide().unwrap();
                };
            };
        })
        .invoke_handler(tauri::generate_handler![start_vpn, stop_vpn, load_ui_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}