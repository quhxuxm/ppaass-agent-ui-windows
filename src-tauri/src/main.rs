// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use std::sync::Arc;

use clap::Parser;
use ppaass_agent::{command::AgentServerCommand, config::AgentServerConfig};
use ppaass_agent::{event::AgentServerEvent, server::AgentServer};
use tauri::{
    CustomMenuItem, Manager, Result, State, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, Window, WindowEvent,
};
use tokio::{runtime::Builder, sync::mpsc::Sender};
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::vo::{AgentServerConfigurationVo, AgentServerEventType, AgentServerEventVo, NetworkStateVo};

mod vo;

const AGENT_SERVER_EVENT: &str = "__AGENT_SERVER_EVENT__";
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
) -> AgentServerConfigurationVo {
    tauri::async_runtime::block_on(async {
        let agent_server_config = state.agent_server_config.lock().await;
        AgentServerConfigurationVo {
            user_token: agent_server_config.user_token().to_owned(),
            proxy_addresses: agent_server_config.proxy_addresses().clone(),
            port: agent_server_config.port(),
        }
    })
}

#[tauri::command(rename_all = "snake_case")]
fn start_agent_server(
    arg: AgentServerConfigurationVo,
    state: State<'_, AgentServerConfigurationUiState>,
    window: Window,
) {
    println!(
        "Receive agent  server configuration ui business object: {:?}",
        arg
    );
    tauri::async_runtime::block_on(async {
        let proxy_addresses = arg.proxy_addresses;
        let port = arg.port;
        let agent_server_config = {
            let mut agent_server_config_lock = state.agent_server_config.lock().await;
            agent_server_config_lock.set_user_token(arg.user_token);
            agent_server_config_lock.set_port(port);
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
        tauri::async_runtime::spawn(async move {
            while let Some(agent_server_event) = agent_server_event_rx.recv().await {
                let agent_server_backend_to_ui_event = match agent_server_event {
                    AgentServerEvent::NetworkState {
                        upload_mb_amount,
                        upload_mb_per_second,
                        download_mb_amount,
                        download_mb_per_second,
                    } => {
                        let network_state_vo = NetworkStateVo {
                            upload_mb_amount,
                            upload_mb_per_second,
                            download_mb_amount,
                            download_mb_per_second,
                        };
                        let content = match serde_json::to_string(&network_state_vo) {
                            Ok(content) => content,
                            Err(e) => {
                                error!("Fail to serialize network state to json because of error: {e:?}");
                                continue;
                            }
                        };
                        AgentServerEventVo {
                            content,
                            event_type: AgentServerEventType::NetworkState,
                        }
                    }
                    AgentServerEvent::ServerStartSuccess(port) => AgentServerEventVo {
                        content: format!("Agent server start success, listening on: {port}"),
                        event_type: AgentServerEventType::StartSuccess,
                    },
                    AgentServerEvent::ServerStartFail {
                        ..
                    } => AgentServerEventVo {
                        content: format!("Agent server start fail, can not listening on: {port}."),
                        event_type: AgentServerEventType::StartFail,
                    },
                    AgentServerEvent::ServerStopSuccess => AgentServerEventVo {
                        content: "Agent server stop success.".to_string(),
                        event_type: AgentServerEventType::StopSuccess,
                    },
                    AgentServerEvent::ServerStopFail {
                        ..
                    } => AgentServerEventVo {
                        content: "Agent server stop fail.".to_string(),
                        event_type: AgentServerEventType::StopFail,
                    },
                    AgentServerEvent::TunnelInitializeSuccess {
                        client_socket_address,
                        src_address,
                        dst_address,
                    } => AgentServerEventVo {
                        content: format!("Tunnel of client [{client_socket_address}] from [{}] to [{}] initialize success.", src_address.map(|v| format!("{v:?}")).unwrap_or(String::new()), dst_address.map(|v| format!("{v:?}")).unwrap_or(String::new())),
                        event_type: AgentServerEventType::LoggingInfo,
                    },
                    AgentServerEvent::TunnelInitializeFail {
                        client_socket_address,
                        src_address,
                        dst_address,
                        ..
                    } => AgentServerEventVo {
                        content: format!("Tunnel of client [{client_socket_address}] from [{}] to [{}] initialize fail.", src_address.map(|v| format!("{v:?}")).unwrap_or(String::new()), dst_address.map(|v| format!("{v:?}")).unwrap_or(String::new())),
                        event_type: AgentServerEventType::LoggingError,
                    },
                    AgentServerEvent::TunnelStartRelay {
                        client_socket_address,
                        src_address,
                        dst_address,
                    } => AgentServerEventVo {
                        content: format!("Tunnel of client [{client_socket_address}] from [{}] to [{}] start relay.", src_address.map(|v| format!("{v:?}")).unwrap_or(String::new()), dst_address.map(|v| format!("{v:?}")).unwrap_or(String::new())),
                        event_type: AgentServerEventType::LoggingInfo,
                    },
                    AgentServerEvent::TunnelClose {
                        client_socket_address,
                        src_address,
                        dst_address,
                    } => AgentServerEventVo {
                        content: format!("Tunnel of client [{client_socket_address}] from [{}] to [{}] closed.", src_address.map(|v| format!("{v:?}")).unwrap_or(String::new()), dst_address.map(|v| format!("{v:?}")).unwrap_or(String::new())),
                        event_type: AgentServerEventType::LoggingWarn,
                    },
                };
                if let Err(e) = window.emit(AGENT_SERVER_EVENT, agent_server_backend_to_ui_event) {
                    error!("Fail to emit agent server event to frontend because of error: {e:?}")
                };
            }
        });
    });
}

#[tauri::command(rename_all = "snake_case")]
fn stop_agent_server(state: State<'_, AgentServerConfigurationUiState>) {
    info!("Going to stop agent server.");
    tauri::async_runtime::block_on(async {
        let agent_server_command_tx_lock = state.agent_server_command_tx.lock().await;
        if let Some(ref agent_server_command_tx) = *agent_server_command_tx_lock {
            if let Err(e) = agent_server_command_tx.send(AgentServerCommand::Stop).await {
                error!("Fail to send agent server command because of error: {e:?}")
            };
        }
    })
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
        CustomMenuItem::new(SYSTEM_TRAY_MENU_ITEM_START_AGENT.to_string(), "Start");
    let stop_menu_item =
        CustomMenuItem::new(SYSTEM_TRAY_MENU_ITEM_STOP_AGENT.to_string(), "Stop");
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
                        stop_agent_server(state.clone());
                    }
                }
                SYSTEM_TRAY_MENU_ITEM_START_AGENT => {
                    if let Some(window) = app.get_window(MAIN_WINDOW_LABEL) {
                        let state = window.state::<AgentServerConfigurationUiState>();
                        let agent_server_config_ui_bo = tauri::async_runtime::block_on(async {
                            let agent_server_config_lock =
                                state.agent_server_config.lock().await;
                            AgentServerConfigurationVo {
                                user_token: agent_server_config_lock.user_token().to_owned(),
                                proxy_addresses: agent_server_config_lock
                                    .proxy_addresses()
                                    .clone(),
                                port: agent_server_config_lock.port(),
                            }
                        });

                        start_agent_server(
                            agent_server_config_ui_bo,
                            state.clone(),
                            window.clone(),
                        );
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
