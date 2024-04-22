// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use clap::Parser;
use ppaass_agent::{command::AgentServerCommand, config::AgentServerConfig};
use ppaass_agent::{event::AgentServerEvent, server::AgentServer};
use ppaass_ui_common::{
    event::{AgentServerBackendToUiEvent, AGENT_SERVER_EVENT},
    payload::AgentServerConfigUiBo,
};

use tauri::{
    CustomMenuItem, Manager, Result, State, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, Window, WindowEvent,
};

use tokio::sync::Mutex;
use tokio::{runtime::Builder, sync::mpsc::Sender};
use tracing::{error, info};

const AGENT_SERVER_UI_RUNTIME_NAME: &str = "__AGENT_SERVER_UI_RUNTIME__";

const SYSTEM_TRAY_MENU_ITEM_START_AGENT: &str = "SYSTEM_TRAY_MENU_ITEM_START_AGENT";
const SYSTEM_TRAY_MENU_ITEM_STOP_AGENT: &str = "SYSTEM_TRAY_MENU_ITEM_STOP_AGENT";
const SYSTEM_TRAY_MENU_ITEM_EXIT: &str = "SYSTEM_TRAY_MENU_ITEM_EXIT";
const MAIN_WINDOW_LABEL: &str = "main";

pub struct AgentServerConfigurationUiState {
    agent_server_config: Arc<Mutex<AgentServerConfig>>,
    agent_server_command_tx: Mutex<Option<Sender<AgentServerCommand>>>,
}

#[tauri::command(rename_all = "snake_case")]
fn load_agent_server_configuration(
    state: State<'_, AgentServerConfigurationUiState>,
) -> AgentServerConfigUiBo {
    tauri::async_runtime::block_on(async {
        let agent_server_config = state.agent_server_config.lock().await;
        AgentServerConfigUiBo {
            user_token: agent_server_config.user_token().to_owned(),
            proxy_address: agent_server_config.proxy_addresses().join(";"),
            listening_port: agent_server_config.port().to_string(),
        }
    })
}

#[tauri::command(rename_all = "snake_case")]
fn start_agent_server(
    config_ui_bo: AgentServerConfigUiBo,
    state: State<'_, AgentServerConfigurationUiState>,
    window: Window,
) -> Result<()> {
    info!(
        "Receive agent  server configuration ui business object: {:?}",
        config_ui_bo
    );
    tauri::async_runtime::block_on(async {
        let proxy_addresses = config_ui_bo
            .proxy_address
            .split(';')
            .map(|item| item.to_string())
            .collect::<Vec<String>>();
        let listening_port = match config_ui_bo.listening_port.parse::<u16>() {
            Ok(listening_port) => listening_port,
            Err(e) => {
                error!("Fail to parse listening port because of error: {e:?}");
                return;
            }
        };
        let agent_server_config = {
            let mut agent_server_config_lock = state.agent_server_config.lock().await;
            agent_server_config_lock.set_user_token(config_ui_bo.user_token);
            agent_server_config_lock.set_port(listening_port);
            agent_server_config_lock.set_proxy_addresses(proxy_addresses);
            Arc::new(agent_server_config_lock.clone())
        };
        let agent_server = match AgentServer::new(agent_server_config) {
            Ok(agent_server) => agent_server,
            Err(e) => {
                error!("Fail to create agent server instance because of error: {e:?}");
                return;
            }
        };
        let (agent_server_command_tx, mut agent_server_event_rx) = agent_server.start();
        {
            let mut agent_server_command_tx_lock = state.agent_server_command_tx.lock().await;
            *agent_server_command_tx_lock = Some(agent_server_command_tx);
        }
        while let Some(agent_server_event) = agent_server_event_rx.recv().await {
            let agent_server_backend_to_ui_event = match agent_server_event {
                AgentServerEvent::NetworkState {
                    upload_mb_amount,
                    upload_mb_per_second,
                    download_mb_amount,
                    download_mb_per_second,
                } => AgentServerBackendToUiEvent::NetworkState {
                    upload_mb_amount,
                    upload_mb_per_second,
                    download_mb_amount,
                    download_mb_per_second,
                },
                AgentServerEvent::ServerStartSuccess(port) => {
                    AgentServerBackendToUiEvent::StartSuccess(port)
                }
                AgentServerEvent::ServerStartFail {
                    listening_port,
                    reason,
                } => AgentServerBackendToUiEvent::StartFail {
                    listening_port,
                    reason,
                },
                AgentServerEvent::ServerStopSuccess => AgentServerBackendToUiEvent::StopSuccess,
                AgentServerEvent::ServerStopFail {
                    listening_port,
                    reason,
                } => AgentServerBackendToUiEvent::StopFail {
                    listening_port,
                    reason,
                },
                AgentServerEvent::TunnelInitializeSuccess {
                    client_socket_address,
                    src_address,
                    dst_address,
                } => AgentServerBackendToUiEvent::Logging {
                    client_socket_address,
                    src_address,
                    dst_address,
                    reason: None,
                },
                AgentServerEvent::TunnelInitializeFail {
                    client_socket_address,
                    src_address,
                    dst_address,
                    reason,
                } => AgentServerBackendToUiEvent::Logging {
                    client_socket_address,
                    src_address,
                    dst_address,
                    reason: Some(reason),
                },
                AgentServerEvent::TunnelStartRelay {
                    client_socket_address,
                    src_address,
                    dst_address,
                } => AgentServerBackendToUiEvent::Logging {
                    client_socket_address,
                    src_address,
                    dst_address,
                    reason: None,
                },
                AgentServerEvent::TunnelClose {
                    client_socket_address,
                    src_address,
                    dst_address,
                } => AgentServerBackendToUiEvent::Logging {
                    client_socket_address,
                    src_address,
                    dst_address,
                    reason: None,
                },
            };
            if let Err(e) = window.emit(AGENT_SERVER_EVENT, agent_server_backend_to_ui_event) {
                error!("Fail to emit agent server event to frontend because of error: {e:?}")
            };
        }
    });
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
fn stop_agent_server(state: State<'_, AgentServerConfigurationUiState>) -> Result<()> {
    info!("Going to stop agent server.");
    tauri::async_runtime::block_on(async {
        let agent_server_command_tx_lock = state.agent_server_command_tx.lock().await;
        if let Some(ref agent_server_command_tx) = *agent_server_command_tx_lock {
            if let Err(e) = agent_server_command_tx.send(AgentServerCommand::Stop).await {
                error!("Fail to send agent server command because of error: {e:?}")
            };
        }
    });
    Ok(())
}

fn main() -> Result<()> {
    let agent_server_config = AgentServerConfig::parse();
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .thread_name(AGENT_SERVER_UI_RUNTIME_NAME)
        .worker_threads(agent_server_config.worker_thread_number())
        .build()?;
    tauri::async_runtime::set(runtime.handle().clone());
    let agent_server_config = Arc::new(Mutex::new(agent_server_config));
    let initial_state = AgentServerConfigurationUiState {
        agent_server_config: agent_server_config.clone(),
        agent_server_command_tx: Mutex::new(None),
    };
    let _log_guard = runtime.block_on(async {
        let agent_server_config_lock = agent_server_config.lock().await;
        ppaass_agent::log::init_log(Arc::new(agent_server_config_lock.clone()))
            .expect("Fail to initialize log")
    });

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
                    info!("Agent server ui is going to exit because of exit system tray clicked.");
                    std::process::exit(0);
                }
                SYSTEM_TRAY_MENU_ITEM_STOP_AGENT => {
                    if let Some(window) = app.get_window(MAIN_WINDOW_LABEL) {
                        let state = window.state::<AgentServerConfigurationUiState>();
                        if let Err(e) = stop_agent_server(state.clone()) {
                            error!("Fail to stop vpn because of error: {e:?}");
                        }
                    }
                }
                SYSTEM_TRAY_MENU_ITEM_START_AGENT => {
                    if let Some(window) = app.get_window(MAIN_WINDOW_LABEL) {
                        let state = window.state::<AgentServerConfigurationUiState>();
                        tauri::async_runtime::block_on(async {
                            let agent_server_config_ui_bo = {
                                let agent_server_config_lock =
                                    state.agent_server_config.lock().await;
                                AgentServerConfigUiBo {
                                    user_token: agent_server_config_lock.user_token().to_owned(),
                                    proxy_address: agent_server_config_lock
                                        .proxy_addresses()
                                        .join(";"),
                                    listening_port: agent_server_config_lock.port().to_string(),
                                }
                            };
                            if let Err(e) = start_agent_server(
                                agent_server_config_ui_bo,
                                state.clone(),
                                window.clone(),
                            ) {
                                error!("Fail to start agent server because of error: {e:?}");
                            };
                        });
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
            load_agent_server_configuration
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
