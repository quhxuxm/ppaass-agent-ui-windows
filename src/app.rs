use std::rc::Rc;

use derive_more::Display;
use gloo::utils::format::JsValueSerdeExt;
use serde_wasm_bindgen::to_value;
use stylist::yew::Global;
use stylist::StyleSource;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use web_sys::{HtmlButtonElement, HtmlTextAreaElement};
use yew::prelude::*;

use ppaass_ui_common::{
    event::AgentEvent,
    payload::{AgentConfigInfo, AgentServerSignalPayload, AgentServerSignalType},
};

use crate::{
    bo::BackendCommandArgumentWrapper,
    components::input_field::{InputField, InputFieldDataType},
};
use crate::{
    bo::{command::BackendCommand, BackendEventWrapper},
    components::container::Container,
};
use crate::{
    components::button::Button,
    components::network_info::NetworkInfo,
    wasm_binding::{invoke_tauri_with_arg, invoke_tauri_without_arg, listen_tauri_event},
};

#[derive(Debug, PartialEq, Eq, Clone, Default, Display)]
enum StatusLevel {
    #[display(fmt = "error")]
    Error,
    #[display(fmt = "info")]
    #[default]
    Info,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct StatusDetail {
    text: String,
    level: StatusLevel,
}

#[derive(Debug, PartialEq, Clone, Default)]
struct NetworkDetail {
    upload_bytes_amount: u64,
    upload_mb_per_second: f64,
    download_bytes_amount: u64,
    download_mb_per_second: f64,
}

#[derive(Debug, PartialEq, Clone, Default)]
struct UiState {
    initialized: bool,
    user_token: String,
    proxy_address: String,
    listening_port: String,
    status_detail: StatusDetail,
    network_detail: NetworkDetail,
}

fn generate_start_btn_callback(
    user_token_input_ref: NodeRef,
    proxy_address_field_ref: NodeRef,
    listening_port_field_ref: NodeRef,
    ui_state: UseStateHandle<UiState>,
) -> Callback<MouseEvent> {
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
                    initialized: true,
                    user_token: config_info.user_token,
                    proxy_address: config_info.proxy_address,
                    listening_port: config_info.listening_port,
                    status_detail: StatusDetail {
                        text: "VPN fail to start.".to_string(),
                        level: StatusLevel::Error,
                    },
                    network_detail: Default::default(),
                };
                ui_state.set(new_ui_state);
            }
        });
    })
}

fn generate_stop_btn_callback() -> Callback<MouseEvent> {
    Callback::from(move |_| {
        spawn_local(async move {
            invoke_tauri_without_arg(BackendCommand::AgentStop.to_string().as_str())
                .await
                .unwrap();
        });
    })
}

pub fn on_register_btn_click(event: MouseEvent) {
    gloo::console::info!("Receive register event: {:?}", event);
}

#[function_component(PpaassAgentUi)]
pub fn ppaass_agent_ui() -> Html {
    let global_style = StyleSource::try_from(include_str!("global.css")).unwrap();
    let user_token_field_ref = use_node_ref();
    let proxy_address_field_ref = use_node_ref();
    let listening_port_field_ref = use_node_ref();
    let start_button_ref = use_node_ref();
    let logging_information_textarea = use_node_ref();
    let ui_state = use_state(UiState::default);

    {
        let user_token_field_ref = user_token_field_ref.clone();
        let proxy_address_field_ref = proxy_address_field_ref.clone();
        let listening_port_field_ref = listening_port_field_ref.clone();
        let start_button_ref = start_button_ref.clone();
        let ui_state = ui_state.clone();
        let logging_information_textarea = logging_information_textarea.clone();
        use_effect(move || {
            //Do the logic when component initialize
            let vpn_start_window_listener = {
                let user_token_field_ref = user_token_field_ref.clone();
                let proxy_address_field_ref = proxy_address_field_ref.clone();
                let listening_port_field_ref = listening_port_field_ref.clone();
                let start_button_ref = start_button_ref.clone();
                let ui_state = ui_state.clone();

                Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
                    let backend_event: BackendEventWrapper<AgentConfigInfo> =
                        event.into_serde().unwrap();
                    let config_info = backend_event.payload;
                    gloo::console::info!(
                        "Receive vpn start window event from backend:",
                        format!("{:?}", config_info.clone())
                    );
                    let user_token_input_field: HtmlInputElement =
                        user_token_field_ref.cast::<HtmlInputElement>().unwrap();

                    let proxy_address_input_field =
                        proxy_address_field_ref.cast::<HtmlInputElement>().unwrap();
                    let listening_port_field =
                        listening_port_field_ref.cast::<HtmlInputElement>().unwrap();
                    let start_button = start_button_ref.cast::<HtmlButtonElement>().unwrap();

                    let new_ui_state = UiState {
                        initialized: true,
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
                    format!("{new_ui_state:?}"));
                    ui_state.set(new_ui_state);

                    gloo::console::info!(
                        "Receive vpn start window event from backend after reset ui state:",
                        format!("{:?}", *ui_state)
                    );
                })
            };

            let vpn_stop_window_listener = {
                let user_token_field_ref = user_token_field_ref.clone();
                let proxy_address_field_ref = proxy_address_field_ref.clone();
                let listening_port_field_ref = listening_port_field_ref.clone();
                let start_button_ref = start_button_ref.clone();
                let ui_state = ui_state.clone();
                Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
                    gloo::console::info!("Receive vpn stop window event from backend:", event);
                    let user_token_input_field =
                        user_token_field_ref.cast::<HtmlInputElement>().unwrap();
                    let proxy_address_input_field =
                        proxy_address_field_ref.cast::<HtmlInputElement>().unwrap();
                    let listening_port_field =
                        listening_port_field_ref.cast::<HtmlInputElement>().unwrap();
                    let start_button = start_button_ref.cast::<HtmlButtonElement>().unwrap();
                    gloo::console::info!(
                        "Receive vpn stop window event from backend and get all fields success"
                    );
                    let new_ui_state = UiState {
                        initialized: true,
                        user_token: ui_state.user_token.clone(),
                        proxy_address: ui_state.proxy_address.clone(),
                        listening_port: ui_state.listening_port.clone(),
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
                    ui_state.set(new_ui_state);
                })
            };

            let agent_signal_listener = {
                let ui_state = ui_state.clone();
                Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
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
                            initialized: true,
                            user_token: ui_state.user_token.clone(),
                            proxy_address: ui_state.proxy_address.clone(),
                            listening_port: ui_state.listening_port.clone(),
                            status_detail: ui_state.status_detail.clone(),
                            network_detail: NetworkDetail {
                                upload_bytes_amount,
                                upload_mb_per_second,
                                download_bytes_amount,
                                download_mb_per_second,
                            },
                        };
                        ui_state.set(new_ui_state);
                        return;
                    }
                    if let AgentServerSignalType::Error = agent_server_signal.signal_type {
                        if let Some(message) = &agent_server_signal.message {
                            let new_ui_state = UiState {
                                initialized: true,
                                user_token: ui_state.user_token.clone(),
                                proxy_address: ui_state.proxy_address.clone(),
                                listening_port: ui_state.listening_port.clone(),
                                status_detail: StatusDetail {
                                    text: message.to_string(),
                                    level: StatusLevel::Error,
                                },
                                network_detail: ui_state.network_detail.clone(),
                            };
                            ui_state.set(new_ui_state);
                        }
                    }

                    let logging_information_textarea = logging_information_textarea
                        .cast::<HtmlTextAreaElement>()
                        .unwrap();
                    let origianl_logging_text_value = logging_information_textarea.value();
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
                    if let Some(message) = &agent_server_signal.message {
                        logging_text_value.push_str(message);
                    }

                    logging_information_textarea.set_value(&logging_text_value);
                    let scroll_height = logging_information_textarea.scroll_height();
                    logging_information_textarea.set_scroll_top(scroll_height);
                })
            };

            let vpn_start_window_listener = Rc::new(vpn_start_window_listener);
            let vpn_stop_window_listener = Rc::new(vpn_stop_window_listener);
            let agent_signal_listener = Rc::new(agent_signal_listener);
            {
                let vpn_start_window_listener = vpn_start_window_listener.clone();
                let vpn_stop_window_listener = vpn_stop_window_listener.clone();
                let agent_signal_listener = agent_signal_listener.clone();
                spawn_local(async move {
                    let _ = listen_tauri_event(
                        AgentEvent::Start.to_string().as_str(),
                        &vpn_start_window_listener,
                    )
                    .await;
                    let _ = listen_tauri_event(
                        AgentEvent::Stop.to_string().as_str(),
                        &vpn_stop_window_listener,
                    )
                    .await;
                    let _ = listen_tauri_event(
                        AgentEvent::Signal.to_string().as_str(),
                        &agent_signal_listener,
                    )
                    .await;
                });
            }

            move || {
                // Do the logic when component destroy
                // The listener must drop here otherwise it will cause multiple listener registered.
                // When listener dropped the event will not be listened.
                drop(vpn_start_window_listener);
                drop(vpn_stop_window_listener);
                drop(agent_signal_listener);
            }
        });
    }

    if !ui_state.initialized {
        let ui_state = ui_state.clone();
        spawn_local(async move {
            let config_info =
                match invoke_tauri_without_arg(BackendCommand::LoadConfigInfo.to_string().as_str())
                    .await
                {
                    Ok(config_info) => config_info,
                    Err(_) => {
                        let new_ui_state = UiState {
                            initialized: true,
                            user_token: "".to_string(),
                            proxy_address: "".to_string(),
                            listening_port: "".to_string(),
                            status_detail: StatusDetail {
                                text: "Agent fail to initialize.".to_string(),
                                level: StatusLevel::Info,
                            },
                            network_detail: Default::default(),
                        };
                        ui_state.set(new_ui_state);
                        return;
                    }
                };

            let config_info: AgentConfigInfo = config_info.into_serde().unwrap();
            gloo::console::info!("Load config info:", format!("{config_info:?}"));
            let new_ui_state = UiState {
                initialized: true,
                user_token: config_info.user_token,
                proxy_address: config_info.proxy_address,
                listening_port: config_info.listening_port,
                status_detail: StatusDetail {
                    text: "Agent initialize.".to_string(),
                    level: StatusLevel::Info,
                },
                network_detail: Default::default(),
            };
            gloo::console::info!("Generate new ui state:", format!("{new_ui_state:?}"));
            ui_state.set(new_ui_state);
            gloo::console::info!("Agent initialized:", format!("{:?}", *ui_state));
        });
        return html! {
            <>
                <Global css={global_style} />
                <div class="loading_message">{"Loading ... "}</div>
            </>
        };
    }

    let status_detail = ui_state.status_detail.clone();
    let user_token = ui_state.user_token.clone();
    let proxy_address = ui_state.proxy_address.clone();
    let listening_port = ui_state.listening_port.clone();
    let upload_network_info = format!(
        "↑ Total: {:.2} MB; Avg {:.2} MB/S",
        ui_state.network_detail.upload_bytes_amount as f64 / (1024 * 1024) as f64,
        ui_state.network_detail.upload_mb_per_second
    );
    let download_network_info = format!(
        "↓ Total: {:.2} MB; Avg: {:.2} MB/S",
        ui_state.network_detail.download_bytes_amount as f64 / (1024 * 1024) as f64,
        ui_state.network_detail.download_mb_per_second
    );
    html! {
        <>
            <Global css={global_style} />
            <div class="left_panel">
                <Container classes="input_field_panel">
                    <InputField id="user_token" label={"User token:"}
                    place_holder={"Enter user token"}
                    hint="Register to get user token" input_ref={&user_token_field_ref} value={user_token}/>
                    <InputField id="proxy_address" label={"Proxy address:"}
                    place_holder={"Enter proxy address"}
                    hint={"Proxy addresses are seperate with \";\""} input_ref={&proxy_address_field_ref} value={proxy_address}/>
                    <InputField id="listening_port" label={"Listening port:"}
                    place_holder={"Enter listening port"}
                    data_type={InputFieldDataType::Number{min: 1025, max: 65535}}
                    hint={"Listening port should between 1025~65536"} input_ref={&listening_port_field_ref} value={listening_port}/>
                </Container>
                <Container classes="button_panel">
                    <Button id="register_button" label="Register" classes="button"
                    on_click={on_register_btn_click} />
                    <Button id="start_button" label="Start" classes="button" button_ref={&start_button_ref}
                    on_click={generate_start_btn_callback(user_token_field_ref.clone(),proxy_address_field_ref.clone(), listening_port_field_ref.clone(), ui_state.clone())} />
                    <Button id="stop_button" label="Stop" classes="button"
                    on_click={generate_stop_btn_callback()} />
                </Container>
                <Container classes="network_panel">
                    <span class="upload">{upload_network_info} </span>
                    <span class="download">{download_network_info} </span>
                </Container>
                <Container classes="status_panel">
                    <span class={["log".to_string(), status_detail.level.to_string()]}>
                        {&*status_detail.text}
                    </span>
                </Container>

            </div>

            <div class="right_panel">
                <Container classes="network_status">
                    <label>{"Network status"}</label>
                    <NetworkInfo></NetworkInfo>
                </Container>
                <Container classes="logging">
                    <label for="logging_textarea">{"Logging information"}</label>
                    <textarea id="logging_textarea" ref={logging_information_textarea} readonly={true}></textarea>
                </Container>
            </div>
        </>
    }
}
