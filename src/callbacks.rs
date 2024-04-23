use gloo::utils::format::JsValueSerdeExt;

use ppaass_agent_ui_model::model::{
    UiModelAgentServerConfiguration, UiModelBackendEvent, UiModelNetworkDetail,
    UiModelStatusBarDetail, UiModelStatusBarDetailType,
};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{closure::Closure, JsValue};
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlTextAreaElement};
use yew::{platform::spawn_local, Callback, MouseEvent, NodeRef, UseStateHandle};

use crate::{
    bo::{UiBackendCommand, UiBackendCommandArgWrapper, UiBackendEventWrapper, UiStateMainPage},
    wasm_binding::{invoke_tauri_with_arg, invoke_tauri_without_arg},
};

#[derive(Clone)]
pub struct UiCallbackParamStartAgentServer {
    pub user_token_input_ref: NodeRef,
    pub proxy_address_input_ref: NodeRef,
    pub listening_port_input_ref: NodeRef,
}

pub fn generate_start_agent_server_btn_callback(
    param: UiCallbackParamStartAgentServer,
) -> Callback<MouseEvent> {
    let UiCallbackParamStartAgentServer {
        user_token_input_ref,
        proxy_address_input_ref,
        listening_port_input_ref,
    } = param;
    Callback::from(move |_| {
        let user_token_input = user_token_input_ref.cast::<HtmlInputElement>().unwrap();
        let proxy_address_input = proxy_address_input_ref.cast::<HtmlInputElement>().unwrap();
        let listening_port_input = listening_port_input_ref.cast::<HtmlInputElement>().unwrap();
        let ui_model_agent_server_config = UiModelAgentServerConfiguration {
            user_token: user_token_input.value(),
            proxy_address: proxy_address_input
                .value()
                .split(';')
                .map(|item| item.to_owned())
                .collect::<Vec<String>>(),
            listening_port: listening_port_input.value().parse::<u16>().unwrap(),
        };
        let backend_command_arg_wrapper = UiBackendCommandArgWrapper {
            arg: ui_model_agent_server_config.clone(),
        };
        spawn_local(async move {
            let backend_command_arg_wrapper_js_value =
                to_value(&backend_command_arg_wrapper).unwrap();
            invoke_tauri_with_arg(
                UiBackendCommand::StartAgentServer.name(),
                backend_command_arg_wrapper_js_value,
            )
            .await
            .unwrap();
        });
    })
}

pub fn generate_stop_agent_server_btn_callback() -> Callback<MouseEvent> {
    Callback::from(move |_| {
        spawn_local(async move {
            invoke_tauri_without_arg(UiBackendCommand::StopAgentServer.name())
                .await
                .unwrap();
        });
    })
}

pub struct UiCallbackParamBackendEventListener {
    pub user_token_input_ref: NodeRef,
    pub proxy_address_input_ref: NodeRef,
    pub listening_port_input_ref: NodeRef,
    pub start_button_ref: NodeRef,
    pub stop_button_ref: NodeRef,
    pub logging_textarea_ref: NodeRef,
    pub main_page_ui_state: UseStateHandle<UiStateMainPage>,
}

pub fn generate_backend_event_listener_callback(
    param: UiCallbackParamBackendEventListener,
) -> Closure<dyn FnMut(JsValue)> {
    let UiCallbackParamBackendEventListener {
        user_token_input_ref,
        proxy_address_input_ref,
        listening_port_input_ref,
        start_button_ref,
        stop_button_ref,
        logging_textarea_ref,
        main_page_ui_state,
    } = param;
    Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
        let user_token_input = user_token_input_ref.cast::<HtmlInputElement>().unwrap();
        let proxy_address_input = proxy_address_input_ref.cast::<HtmlInputElement>().unwrap();
        let listening_port_input = listening_port_input_ref.cast::<HtmlInputElement>().unwrap();
        let start_button = start_button_ref.cast::<HtmlButtonElement>().unwrap();
        let stop_button = stop_button_ref.cast::<HtmlButtonElement>().unwrap();
        let logging_textarea = logging_textarea_ref.cast::<HtmlTextAreaElement>().unwrap();

        let backend_to_ui_event_wrapper: UiBackendEventWrapper<UiModelBackendEvent> =
            event.into_serde().unwrap();
        let backend_to_ui_event = backend_to_ui_event_wrapper.payload;

        match backend_to_ui_event {
            UiModelBackendEvent::StartSuccess(port) => {
                proxy_address_input.set_disabled(true);
                user_token_input.set_disabled(true);
                listening_port_input.set_disabled(true);
                start_button.set_disabled(true);
                stop_button.set_disabled(false);
                let new_main_page_ui_state = UiStateMainPage {
                    configuration: Some(UiModelAgentServerConfiguration {
                        user_token: user_token_input.value(),
                        proxy_address: proxy_address_input
                            .value()
                            .split(';')
                            .map(|item| item.to_owned())
                            .collect::<Vec<String>>(),
                        listening_port: listening_port_input.value().parse::<u16>().unwrap(),
                    }),
                    status_bar_detail: UiModelStatusBarDetail {
                        text: format!("Agent server started success, listning on port: {port}."),
                        level: UiModelStatusBarDetailType::Info,
                    },
                    ..(*main_page_ui_state).clone()
                };
                main_page_ui_state.set(new_main_page_ui_state);
                gloo::console::info!(
                    "Receive agent server started success window event from backend."
                );
            }
            UiModelBackendEvent::StartFail { reason, .. } => {
                let new_main_page_ui_state = UiStateMainPage {
                    status_bar_detail: UiModelStatusBarDetail {
                        text: reason,
                        level: UiModelStatusBarDetailType::Error,
                    },
                    ..(*main_page_ui_state).clone()
                };
                main_page_ui_state.set(new_main_page_ui_state);
                gloo::console::info!(
                    "Receive agent server started fail window event from backend."
                );
            }
            UiModelBackendEvent::StopSuccess => {
                stop_button.set_disabled(true);
                proxy_address_input.set_disabled(false);
                user_token_input.set_disabled(false);
                listening_port_input.set_disabled(false);
                start_button.set_disabled(false);
                let new_main_page_ui_state = UiStateMainPage {
                    status_bar_detail: UiModelStatusBarDetail {
                        text: "Agent server stopped.".to_string(),
                        level: UiModelStatusBarDetailType::Info,
                    },
                    ..(*main_page_ui_state).clone()
                };
                main_page_ui_state.set(new_main_page_ui_state);

                gloo::console::info!(
                    "Receive agent server stopped success window event from backend."
                );
            }
            UiModelBackendEvent::StopFail { reason, .. } => {
                let new_main_page_ui_state = UiStateMainPage {
                    status_bar_detail: UiModelStatusBarDetail {
                        text: reason,
                        level: UiModelStatusBarDetailType::Error,
                    },
                    ..(*main_page_ui_state).clone()
                };
                main_page_ui_state.set(new_main_page_ui_state);
                gloo::console::info!(
                    "Receive agent server stopped fail window event from backend."
                );
            }
            UiModelBackendEvent::Logging { reason, .. } => {
                if let Some(reason) = reason {
                    let origianl_logging_text_value = logging_textarea.value();
                    let all_original_logging_lines = origianl_logging_text_value
                        .split("\n\n")
                        .collect::<Vec<&str>>();
                    let mut start_index = all_original_logging_lines.len() as isize - 1000;
                    if start_index < 0 {
                        start_index = 0;
                    }
                    let all_original_logging_lines =
                        &all_original_logging_lines[start_index as usize..];
                    let mut logging_text_value = all_original_logging_lines.join("\n\n");
                    logging_text_value.push_str("\n\n");

                    logging_text_value.push_str(&reason);

                    logging_textarea.set_value(&logging_text_value);
                    let scroll_height = logging_textarea.scroll_height();
                    logging_textarea.set_scroll_top(scroll_height);
                }
            }
            UiModelBackendEvent::NetworkState {
                upload_mb_amount,
                upload_mb_per_second,
                download_mb_amount,
                download_mb_per_second,
            } => {
                let mut current_network_chart_download_mb_per_second = main_page_ui_state
                    .network_chart_download_mb_per_second
                    .clone();
                current_network_chart_download_mb_per_second.push_back(download_mb_per_second);
                if current_network_chart_download_mb_per_second.len() > 30 {
                    current_network_chart_download_mb_per_second.pop_front();
                }

                let mut current_network_chart_upload_mb_per_second = main_page_ui_state
                    .network_chart_upload_mb_per_second
                    .clone();
                current_network_chart_upload_mb_per_second.push_back(upload_mb_per_second);
                if current_network_chart_upload_mb_per_second.len() > 30 {
                    current_network_chart_upload_mb_per_second.pop_front();
                }

                let new_main_page_ui_state = UiStateMainPage {
                    network_detail: UiModelNetworkDetail {
                        upload_mb_amount,
                        upload_mb_per_second,
                        download_mb_amount,
                        download_mb_per_second,
                    },
                    network_chart_download_mb_per_second:
                        current_network_chart_download_mb_per_second,
                    network_chart_upload_mb_per_second: current_network_chart_upload_mb_per_second,
                    ..(*main_page_ui_state).clone()
                };
                main_page_ui_state.set(new_main_page_ui_state);
            }
        }
    })
}
