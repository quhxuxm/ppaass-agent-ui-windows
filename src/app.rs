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

use crate::{
    bo::{payload::AgentServerSignal, BackendEvent},
    components::container::Container,
};
use crate::{
    bo::{
        payload::{AgentConfigInfo, AgentServerSignalLevel},
        FrontendArgument,
    },
    components::input_field::{InputField, InputFieldDataType},
};
use crate::{
    components::button::Button,
    wasm_binding::{invoke_tauri_with_arg, invoke_tauri_without_arg, listen_tauri_event},
};

#[derive(Debug, PartialEq, Eq, Clone, Default, Display)]
enum StatusLevel {
    #[display(fmt = "error")]
    Error,
    #[display(fmt = "info")]
    #[default]
    Info,
    #[display(fmt = "warning")]
    Warning,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct StatusDetail {
    text: String,
    level: StatusLevel,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct UiState {
    initialized: bool,
    user_token: String,
    proxy_address: String,
    listening_port: String,
    status_detail: StatusDetail,
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
        let ui_arg = FrontendArgument {
            arg: config_info.clone(),
        };
        let ui_state = ui_state.clone();
        spawn_local(async move {
            let args = to_value(&ui_arg).unwrap();
            if (invoke_tauri_with_arg("start_vpn", args).await).is_err() {
                let new_ui_state = UiState {
                    initialized: true,
                    user_token: config_info.user_token,
                    proxy_address: config_info.proxy_address,
                    listening_port: config_info.listening_port,
                    status_detail: StatusDetail {
                        text: "VPN fail to start.".to_string(),
                        level: StatusLevel::Error,
                    },
                };
                ui_state.set(new_ui_state);
            }
        });
    })
}

fn generate_stop_btn_callback() -> Callback<MouseEvent> {
    Callback::from(move |_| {
        spawn_local(async move {
            invoke_tauri_without_arg("stop_vpn").await.unwrap();
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
                    let backend_event: BackendEvent<AgentConfigInfo> = event.into_serde().unwrap();
                    let config_info = backend_event.payload;
                    gloo::console::info!(
                        "Receive vpn start window event from backend:",
                        format!("{:?}", config_info.clone())
                    );
                    let user_token_input_field =
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
                    let backend_event: BackendEvent<AgentServerSignal> =
                        event.into_serde().unwrap();
                    let agent_server_signal = backend_event.payload;
                    let new_ui_state = UiState {
                        initialized: true,
                        user_token: ui_state.user_token.clone(),
                        proxy_address: ui_state.proxy_address.clone(),
                        listening_port: ui_state.listening_port.clone(),
                        status_detail: StatusDetail {
                            text: agent_server_signal.message.clone(),
                            level: match agent_server_signal.level {
                                AgentServerSignalLevel::Info => StatusLevel::Info,
                                AgentServerSignalLevel::Error => StatusLevel::Error,
                            },
                        },
                    };
                    ui_state.set(new_ui_state);
                    let logging_information_textarea = logging_information_textarea
                        .cast::<HtmlTextAreaElement>()
                        .unwrap();
                    let origianl_logging_text_value = logging_information_textarea.value();
                    let all_original_logging_lines =
                        origianl_logging_text_value.lines().collect::<Vec<&str>>();
                    let mut start_index = all_original_logging_lines.len() as isize - 100;
                    if start_index < 0 {
                        start_index = 0;
                    }
                    let all_original_logging_lines =
                        &all_original_logging_lines[start_index as usize..];
                    let mut logging_text_value = all_original_logging_lines.join("\n");
                    logging_text_value.push('\n');
                    logging_text_value.push_str(&agent_server_signal.message);
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
                    let _ = listen_tauri_event("vpnstart", &vpn_start_window_listener).await;
                    let _ = listen_tauri_event("vpnstop", &vpn_stop_window_listener).await;
                    let _ = listen_tauri_event("vpnsignal", &agent_signal_listener).await;
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
            let config_info = match invoke_tauri_without_arg("load_ui_info").await {
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

    html! {
        <>
            <Global css={global_style} />
            <Container classes="left_panel">
                <Container classes="input_field_panel">
                    <InputField id="user_token" label={"User token:"}
                    place_holder={"Enter user token"}
                    hint="Register to get user token" input_ref={&user_token_field_ref} value={user_token}/>
                    <InputField id="proxy_address" label={"Proxy address:"}
                    place_holder={"Enter proxy address"}
                    hint={"Proxy addresses are seperate with \";\""} input_ref={&proxy_address_field_ref} value={proxy_address}/>
                    <InputField id="listening_port" label={"Listening port:"}
                    place_holder={"Enter listening port"}
                    data_type={InputFieldDataType::Number{min: 0, max: 65535}}
                    hint={"Listening port should between 0~65536"} input_ref={&listening_port_field_ref} value={listening_port}/>
                </Container>
                <Container classes="button_panel">
                    <Button id="register_button" label="Register" classes="button"
                    on_click={on_register_btn_click} />
                    <Button id="start_button" label="Start" classes="button" button_ref={&start_button_ref}
                    on_click={generate_start_btn_callback(user_token_field_ref.clone(),proxy_address_field_ref.clone(), listening_port_field_ref.clone(), ui_state.clone())} />
                    <Button id="stop_button" label="Stop" classes="button"
                    on_click={generate_stop_btn_callback()} />
                </Container>
                <Container classes="status_panel">
                    <span class={status_detail.level.to_string()}>{&*status_detail.text}</span>
                </Container>
            </Container>
            <Container classes="right_panel">
                <label for="logging_textarea">{"Logging information "}</label>
                <textarea id="logging_textarea" ref={logging_information_textarea} readonly={true}></textarea>
            </Container>
        </>
    }
}
