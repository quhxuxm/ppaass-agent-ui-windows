use std::collections::VecDeque;

use gloo::utils::format::JsValueSerdeExt;
use ppaass_ui_common::payload::{AgentConfigInfo, AgentServerSignalPayload, AgentServerSignalType};

use serde_wasm_bindgen::to_value;
use wasm_bindgen::{closure::Closure, JsValue};
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlTextAreaElement};
use yew::{platform::spawn_local, Callback, MouseEvent, NodeRef, UseStateHandle};

use crate::{
    bo::{
        command::BackendCommand,
        ui_state::{NetworkDetail, StatusDetail, StatusLevel, UiState},
        BackendCommandArgumentWrapper, BackendEventWrapper,
    },
    wasm_binding::{invoke_tauri_with_arg, invoke_tauri_without_arg},
};

pub struct StartBtnCallbackParam {
    pub user_token_input_ref: NodeRef,
    pub proxy_address_field_ref: NodeRef,
    pub listening_port_field_ref: NodeRef,
    pub ui_state: UseStateHandle<Option<UiState>>,
}

pub fn generate_start_btn_callback(param: StartBtnCallbackParam) -> Callback<MouseEvent> {
    let StartBtnCallbackParam {
        user_token_input_ref,
        proxy_address_field_ref,
        listening_port_field_ref,
        ui_state,
    } = param;
    Callback::from(move |_| {
        let user_token_input_field = user_token_input_ref.cast::<HtmlInputElement>().unwrap();
        let proxy_address_input_field = proxy_address_field_ref.cast::<HtmlInputElement>().unwrap();
        let listening_port_field = listening_port_field_ref.cast::<HtmlInputElement>().unwrap();

        let config_info = AgentConfigInfo {
            user_token: user_token_input_field.value(),
            proxy_address: proxy_address_input_field.value(),
            listening_port: listening_port_field.value(),
        };
        let ui_arg = BackendCommandArgumentWrapper {
            arg: config_info.clone(),
        };
        let ui_state = ui_state.clone();
        spawn_local(async move {
            let args = to_value(&ui_arg).unwrap();
            if (invoke_tauri_with_arg(BackendCommand::AgentStart.to_string().as_str(), args).await)
                .is_err()
            {
                let new_ui_state = UiState {
                    user_token: config_info.user_token,
                    proxy_address: config_info.proxy_address,
                    listening_port: config_info.listening_port,
                    status_detail: StatusDetail {
                        text: "VPN fail to start.".to_string(),
                        level: StatusLevel::Error,
                    },
                    network_detail: Default::default(),
                };
                ui_state.set(Some(new_ui_state));
            }
        });
    })
}

pub fn generate_stop_btn_callback() -> Callback<MouseEvent> {
    Callback::from(move |_| {
        spawn_local(async move {
            invoke_tauri_without_arg(BackendCommand::AgentStop.to_string().as_str())
                .await
                .unwrap();
        });
    })
}

pub struct AgentServerStartedCallbackParam {
    pub user_token_input_ref: NodeRef,
    pub proxy_address_field_ref: NodeRef,
    pub listening_port_field_ref: NodeRef,
    pub start_button_ref: NodeRef,
    pub ui_state: UseStateHandle<Option<UiState>>,
}

pub fn generate_agent_server_started_callback(
    param: AgentServerStartedCallbackParam,
) -> Closure<dyn FnMut(JsValue)> {
    let AgentServerStartedCallbackParam {
        user_token_input_ref,
        proxy_address_field_ref,
        listening_port_field_ref,
        start_button_ref,
        ui_state,
    } = param;
    Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
        let backend_event: BackendEventWrapper<AgentConfigInfo> = event.into_serde().unwrap();
        let config_info = backend_event.payload;
        gloo::console::info!(
            "Receive vpn start window event from backend:",
            format!("{:?}", config_info.clone())
        );
        let user_token_input_field: HtmlInputElement =
            user_token_input_ref.cast::<HtmlInputElement>().unwrap();

        let proxy_address_input_field = proxy_address_field_ref.cast::<HtmlInputElement>().unwrap();
        let listening_port_field = listening_port_field_ref.cast::<HtmlInputElement>().unwrap();
        let start_button = start_button_ref.cast::<HtmlButtonElement>().unwrap();

        let new_ui_state = UiState {
            user_token: config_info.user_token,
            proxy_address: config_info.proxy_address,
            listening_port: config_info.listening_port.clone(),
            status_detail: StatusDetail {
                text: format!(
                    "VPN started, listening on port: {}",
                    config_info.listening_port
                ),
                level: StatusLevel::Info,
            },
            network_detail: Default::default(),
        };
        proxy_address_input_field.set_disabled(true);
        user_token_input_field.set_disabled(true);
        listening_port_field.set_disabled(true);
        start_button.set_disabled(true);
        gloo::console::info!(
            "Receive vpn start window event from backend and going to reset ui with new state:",
            format!("{new_ui_state:?}")
        );
        ui_state.set(Some(new_ui_state));

        gloo::console::info!(
            "Receive vpn start window event from backend after reset ui state:",
            format!("{:?}", *ui_state)
        );
    })
}

pub struct AgentServerStopCallbackParam {
    pub user_token_input_ref: NodeRef,
    pub proxy_address_field_ref: NodeRef,
    pub listening_port_field_ref: NodeRef,
    pub start_button_ref: NodeRef,
    pub ui_state: UseStateHandle<Option<UiState>>,
}

pub fn generate_agent_server_stop_callback(
    param: AgentServerStopCallbackParam,
) -> Closure<dyn FnMut(JsValue)> {
    let AgentServerStopCallbackParam {
        user_token_input_ref,
        proxy_address_field_ref,
        listening_port_field_ref,
        start_button_ref,
        ui_state,
    } = param;
    Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| match *ui_state {
        None => (),
        Some(ref ui_state_inner) => {
            gloo::console::info!("Receive vpn stop window event from backend:", event);
            let user_token_input_field = user_token_input_ref.cast::<HtmlInputElement>().unwrap();
            let proxy_address_input_field =
                proxy_address_field_ref.cast::<HtmlInputElement>().unwrap();
            let listening_port_field = listening_port_field_ref.cast::<HtmlInputElement>().unwrap();
            let start_button = start_button_ref.cast::<HtmlButtonElement>().unwrap();
            gloo::console::info!(
                "Receive vpn stop window event from backend and get all fields success"
            );
            let new_ui_state = UiState {
                user_token: ui_state_inner.user_token.clone(),
                proxy_address: ui_state_inner.proxy_address.clone(),
                listening_port: ui_state_inner.listening_port.clone(),
                status_detail: StatusDetail {
                    text: "VPN stopped.".to_string(),
                    level: StatusLevel::Info,
                },
                network_detail: Default::default(),
            };
            gloo::console::info!(
                "Receive vpn stop window event from backend and going to reset ui state:",
                format!("{new_ui_state:?}")
            );
            proxy_address_input_field.set_disabled(false);
            user_token_input_field.set_disabled(false);
            listening_port_field.set_disabled(false);
            start_button.set_disabled(false);
            ui_state.set(Some(new_ui_state));
        }
    })
}

pub struct AgentServerSignalCallbackParam {
    pub logging_textarea: NodeRef,
    pub ui_state: UseStateHandle<Option<UiState>>,
    pub network_info_download_content_data: UseStateHandle<VecDeque<String>>,
    pub network_info_upload_content_data: UseStateHandle<VecDeque<String>>,
}

pub fn generate_agent_server_signal_callback(
    param: AgentServerSignalCallbackParam,
) -> Closure<dyn FnMut(JsValue)> {
    let AgentServerSignalCallbackParam {
        logging_textarea,

        ui_state,
        network_info_download_content_data,
        network_info_upload_content_data,
    } = param;
    Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| match *ui_state {
        None => (),
        Some(ref ui_state_inner) => {
            let backend_event: BackendEventWrapper<AgentServerSignalPayload> =
                event.into_serde().unwrap();
            let agent_server_signal = backend_event.payload;
            if let AgentServerSignalType::NetworkInfo {
                upload_bytes_amount,
                upload_mb_per_second,
                download_bytes_amount,
                download_mb_per_second,
            } = agent_server_signal.signal_type
            {
                let new_ui_state = UiState {
                    user_token: ui_state_inner.user_token.clone(),
                    proxy_address: ui_state_inner.proxy_address.clone(),
                    listening_port: ui_state_inner.listening_port.clone(),
                    status_detail: ui_state_inner.status_detail.clone(),
                    network_detail: NetworkDetail {
                        upload_bytes_amount,
                        upload_mb_per_second,
                        download_bytes_amount,
                        download_mb_per_second,
                    },
                };
                ui_state.set(Some(new_ui_state));
                let mut current_network_info_download_content_data =
                    (*network_info_download_content_data).clone();
                current_network_info_download_content_data
                    .push_back(format!("{download_mb_per_second:.2}"));
                if current_network_info_download_content_data.len() > 30 {
                    current_network_info_download_content_data.pop_front();
                }
                network_info_download_content_data.set(current_network_info_download_content_data);

                let mut current_network_info_upload_content_data =
                    (*network_info_upload_content_data).clone();
                current_network_info_upload_content_data
                    .push_back(format!("{upload_mb_per_second:.2}"));
                if current_network_info_upload_content_data.len() > 30 {
                    current_network_info_upload_content_data.pop_front();
                }
                network_info_upload_content_data.set(current_network_info_upload_content_data);
                return;
            }
            if let AgentServerSignalType::Error = agent_server_signal.signal_type {
                if let Some(message) = &agent_server_signal.message {
                    let new_ui_state = UiState {
                        user_token: ui_state_inner.user_token.clone(),
                        proxy_address: ui_state_inner.proxy_address.clone(),
                        listening_port: ui_state_inner.listening_port.clone(),
                        status_detail: StatusDetail {
                            text: message.to_string(),
                            level: StatusLevel::Error,
                        },
                        network_detail: ui_state_inner.network_detail.clone(),
                    };
                    ui_state.set(Some(new_ui_state));
                }
            }

            let logging_textarea = logging_textarea.cast::<HtmlTextAreaElement>().unwrap();
            let origianl_logging_text_value = logging_textarea.value();
            let all_original_logging_lines = origianl_logging_text_value
                .split("\n\n")
                .collect::<Vec<&str>>();
            let mut start_index = all_original_logging_lines.len() as isize - 1000;
            if start_index < 0 {
                start_index = 0;
            }
            let all_original_logging_lines = &all_original_logging_lines[start_index as usize..];
            let mut logging_text_value = all_original_logging_lines.join("\n\n");
            logging_text_value.push_str("\n\n");
            if let Some(message) = &agent_server_signal.message {
                logging_text_value.push_str(message);
            }
            logging_textarea.set_value(&logging_text_value);
            let scroll_height = logging_textarea.scroll_height();
            logging_textarea.set_scroll_top(scroll_height);
        }
    })
}
