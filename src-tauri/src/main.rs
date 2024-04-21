// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use ppaass_agent::server::{AgentServer, AgentServerGuard};
use ppaass_agent::{command::AgentServerCommand, config::AgentServerConfig};
use ppaass_ui_common::{error::AgentServerUiError, payload::AgentServerConfigInfo};

use tauri::{
    CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, Window, WindowEvent,
};

use tokio::sync::Mutex;
use tracing::error;

const AGENT_SERVER_EVENT: &str = "__AGENT_SERVER_EVENT__";

const SYSTEM_TRAY_MENU_ITEM_START_AGENT: &str = "SYSTEM_TRAY_MENU_ITEM_START_AGENT";
const SYSTEM_TRAY_MENU_ITEM_STOP_AGENT: &str = "SYSTEM_TRAY_MENU_ITEM_STOP_AGENT";
const SYSTEM_TRAY_MENU_ITEM_EXIT: &str = "SYSTEM_TRAY_MENU_ITEM_EXIT";
const MAIN_WINDOW_LABEL: &str = "main";

pub struct AgentFrontendState {
    config_info: AgentServerConfigInfo,
    agent_server_guard: Mutex<Option<AgentServerGuard>>,
}

#[tauri::command(rename_all = "snake_case")]
fn load_ui_info(state: State<'_, AgentFrontendState>) -> AgentServerConfigInfo {
    state.config_info.clone()
}

#[tauri::command(rename_all = "snake_case")]
fn start_agent_server(
    arg: AgentServerConfigInfo,
    state: State<'_, AgentFrontendState>,
    window: Window,
) -> Result<(), AgentServerUiError> {
    println!("Receive agent ui info: {:?}", arg);
    let config_info = arg.clone();
    let proxy_addresses = config_info
        .proxy_address
        .split(';')
        .map(|item| item.to_string())
        .collect::<Vec<String>>();
    let listening_port = config_info.listening_port.parse::<u16>()?;
    let mut current_server_config = AgentServerConfig::parse();
    current_server_config.set_user_token(config_info.user_token.clone());
    current_server_config.set_proxy_addresses(proxy_addresses);
    current_server_config.set_port(listening_port);
    let agent_server = AgentServer::new(current_server_config)?;
    let (agent_server_guard, mut agent_server_event_rx) = agent_server.start();

    tauri::async_runtime::spawn(async move {
        let agent_server_guard_lock = state.agent_server_guard.lock().await;
        *agent_server_guard_lock = Some(agent_server_guard);
        while let Some(agent_server_event) = agent_server_event_rx.recv().await {
            if let Err(e) = window.emit(AGENT_SERVER_EVENT, agent_server_event) {
                error!("Fail to emit agent server envent to window because of error: {e:?}")
            };
        }
    });
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
fn stop_agent_server(state: State<'_, AgentFrontendState>) -> Result<(), AgentServerUiError> {
    println!("Going to stop agent server");
    tauri::async_runtime::spawn(async {
        let agent_server_guard_lock = state.agent_server_guard.lock().await;
        if let Some(ref agent_server_guard) = *agent_server_guard_lock {
            agent_server_guard
                .send_server_command(AgentServerCommand::Stop)
                .await;
        }
    });

    Ok(())
}

fn main() {
    let current_server_config = AgentServerConfig::parse();
    let _log_guard =
        ppaass_agent::log::init_log(&current_server_config).expect("Fail to initialize log");
    let initial_state = AgentFrontendState {
        config_info: AgentServerConfigInfo {
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
                        let state = window.state::<AgentFrontendState>();
                        if let Err(e) = stop_agent_server(state.clone()) {
                            error!("Fail to stop vpn because of error: {e:?}");
                        }
                    }
                }
                SYSTEM_TRAY_MENU_ITEM_START_AGENT => {
                    if let Some(window) = app.get_window(MAIN_WINDOW_LABEL) {
                        let state = window.state::<AgentFrontendState>();
                        let config_info = state.config_info.clone();
                        if let Err(e) =
                            start_agent_server(config_info, state.clone(), window.clone())
                        {
                            error!("Fail to start vpn because of error: {e:?}");
                        };
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
        .invoke_handler(tauri::generate_handler![
            start_agent_server,
            stop_agent_server,
            load_ui_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
