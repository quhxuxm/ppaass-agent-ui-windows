use derive_more::Display;
use gloo::utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use stylist::{StyleSource, yew::Global};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlButtonElement;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::{
    button::Button,
    container::Container,
    input_field::{InputField, InputFieldDataType},
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name ="invoke", catch)]
    async fn invoke_with_arg(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name ="invoke", catch)]
    async fn invoke_without_arg(cmd: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name ="invoke", catch)]
    async fn load_ui_info(cmd: &str) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
struct UiArg {
    #[serde(rename = "config_info")]
    config_info: AgentConfigInfo,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
struct AgentConfigInfo {
    user_token: String,
    proxy_address: String,
    listening_port: String,
}

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
    start_button_ref: NodeRef,
    ui_state: UseStateHandle<UiState>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let user_token_input_field = user_token_input_ref.cast::<HtmlInputElement>().unwrap();
        let proxy_address_input_field = proxy_address_field_ref.cast::<HtmlInputElement>().unwrap();
        let listening_port_field = listening_port_field_ref.cast::<HtmlInputElement>().unwrap();
        let start_button = start_button_ref.cast::<HtmlButtonElement>().unwrap();
        let config_info = AgentConfigInfo {
            user_token: user_token_input_field.value(),
            proxy_address: proxy_address_input_field.value(),
            listening_port: listening_port_field.value(),
        };
        let ui_arg = UiArg {
            config_info: config_info.clone(),
        };
        let ui_state = ui_state.clone();
        spawn_local(async move {
            let args = to_value(&ui_arg).unwrap();
            match invoke_with_arg("start_vpn", args).await {
                Ok(_) => {
                    let new_ui_state = UiState {
                        initialized: true,
                        user_token: config_info.user_token,
                        proxy_address: config_info.proxy_address,
                        listening_port: config_info.listening_port,
                        status_detail: StatusDetail {
                            text: "VPN started.".to_string(),
                            level: StatusLevel::Info,
                        },
                    };
                    ui_state.set(new_ui_state);
                }
                Err(_) => {
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
                    return;
                }
            };
            proxy_address_input_field.set_disabled(true);
            user_token_input_field.set_disabled(true);
            listening_port_field.set_disabled(true);
            start_button.set_disabled(true);
        });
    })
}

fn generate_stop_btn_callback(
    user_token_input_ref: NodeRef,
    proxy_address_field_ref: NodeRef,
    listening_port_field_ref: NodeRef,
    start_button_ref: NodeRef,
    ui_state: UseStateHandle<UiState>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let user_token_input_field = user_token_input_ref.cast::<HtmlInputElement>().unwrap();
        let proxy_address_input_field = proxy_address_field_ref.cast::<HtmlInputElement>().unwrap();
        let listening_port_field = listening_port_field_ref.cast::<HtmlInputElement>().unwrap();
        let start_button = start_button_ref.cast::<HtmlButtonElement>().unwrap();
        let ui_state = ui_state.clone();
        spawn_local(async move {
            invoke_without_arg("stop_vpn").await.unwrap();
            proxy_address_input_field.set_disabled(false);
            user_token_input_field.set_disabled(false);
            listening_port_field.set_disabled(false);
            start_button.set_disabled(false);
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
            ui_state.set(new_ui_state);
        });
    })
}

pub fn on_register_btn_click(event: MouseEvent) {
    gloo::console::info!("Receive register event: {:?}", event);
}

#[function_component(PpaassAgentUi)]
pub fn ppaass_agent_ui() -> Html {
    let ui_state = use_state(UiState::default);

    if !ui_state.initialized {
        let ui_state = ui_state.clone();
        spawn_local(async move {
            let config_info = match load_ui_info("load_ui_info").await {
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
            gloo::console::info!("Agent initalized:", format!("{:?}", *ui_state));
        });
    }

    let global_style = StyleSource::try_from(include_str!("global.css")).unwrap();
    let user_name_field_ref = NodeRef::default();
    let proxy_address_field_ref = NodeRef::default();
    let listening_port_field_ref = NodeRef::default();
    let start_button_ref = NodeRef::default();

    let status_detail = ui_state.status_detail.clone();
    if !ui_state.initialized {
        return html! {};
    }
    html! {
        <>
            <Global css={global_style} />
            <h1>{"Ppaass Agent"}</h1>
            <Container classes="input_field_panel">
                <InputField id="user_token" label={"User token:"}
                place_holder={"Enter user token"}
                hint="Register to get user token" input_ref={&user_name_field_ref} value={ui_state.user_token.clone()}/>
                <InputField id="proxy_address" label={"Proxy address:"}
                place_holder={"Enter proxy address"}
                hint={"Proxy addresses are seperate with \";\""} input_ref={&proxy_address_field_ref} value={ui_state.proxy_address.clone()}/>
                <InputField id="listening_port" label={"Listening port:"}
                place_holder={"Enter listening port"}
                data_type={InputFieldDataType::Number{min: 0, max: 65535}}
                hint={"Listening port should between 0~65536"} input_ref={&listening_port_field_ref} value={ui_state.listening_port.clone()}/>
            </Container>
            <Container classes="button_panel">
                <Button id="register_button" label="Register" classes="button"
                on_click={on_register_btn_click} />
                <Button id="start_button" label="Start" classes="button" button_ref={&start_button_ref}
                on_click={generate_start_btn_callback(user_name_field_ref.clone(),proxy_address_field_ref.clone(), listening_port_field_ref.clone(), start_button_ref.clone(), ui_state.clone())} />
                <Button id="stop_button" label="Stop" classes="button"
                on_click={generate_stop_btn_callback(user_name_field_ref,proxy_address_field_ref, listening_port_field_ref, start_button_ref, ui_state.clone())} />
            </Container>
            <Container classes="status_panel">
                <span class={status_detail.level.to_string()}>{&*status_detail.text}</span>
            </Container>
        </>
    }
}
