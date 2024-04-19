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
