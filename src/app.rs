use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use stylist::{StyleSource, yew::Global};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlButtonElement;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::{button::Button, container::Container, input_field::InputField};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name ="invoke")]
    async fn invoke_with_arg(cmd: &str, args: JsValue);

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name ="invoke")]
    async fn invoke_without_arg(cmd: &str);

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name ="invoke")]
    async fn load_ui_info(cmd: &str) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct AgentUiInfoArg {
    info: AgentUiInfo,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct AgentUiInfo {
    #[serde(rename = "user_token")]
    user_token: String,
    #[serde(rename = "proxy_address")]
    proxy_address: String,
    #[serde(rename = "listening_port")]
    listening_port: String,
}

pub fn generate_start_btn_callback(
    user_token_input_ref: NodeRef,
    proxy_address_field_ref: NodeRef,
    listening_port_field_ref: NodeRef,
    start_button_ref: NodeRef,
) -> Callback<web_sys::MouseEvent> {
    Callback::from(move |_| {
        let user_token_input_field = user_token_input_ref.cast::<HtmlInputElement>().unwrap();
        let proxy_address_input_field = proxy_address_field_ref.cast::<HtmlInputElement>().unwrap();
        let listening_port_field = listening_port_field_ref.cast::<HtmlInputElement>().unwrap();
        let start_button = start_button_ref.cast::<HtmlButtonElement>().unwrap();
        let agent_ui_info_arg = AgentUiInfoArg {
            info: AgentUiInfo {
                user_token: user_token_input_field.value(),
                proxy_address: proxy_address_input_field.value(),
                listening_port: listening_port_field.value(),
            },
        };

        spawn_local(async move {
            let args = to_value(&agent_ui_info_arg).unwrap();
            invoke_with_arg("start_vpn", args).await;
            proxy_address_input_field.set_disabled(true);
            user_token_input_field.set_disabled(true);
            listening_port_field.set_disabled(true);
            start_button.set_disabled(true);
        });
    })
}

pub fn generate_stop_btn_callback(
    user_token_input_ref: NodeRef,
    proxy_address_field_ref: NodeRef,
    listening_port_field_ref: NodeRef,
    start_button_ref: NodeRef,
) -> Callback<web_sys::MouseEvent> {
    Callback::from(move |_| {
        let user_token_input_field = user_token_input_ref.cast::<HtmlInputElement>().unwrap();
        let proxy_address_input_field = proxy_address_field_ref.cast::<HtmlInputElement>().unwrap();
        let listening_port_field = listening_port_field_ref.cast::<HtmlInputElement>().unwrap();
        let start_button = start_button_ref.cast::<HtmlButtonElement>().unwrap();
        spawn_local(async move {
            invoke_without_arg("stop_vpn").await;
            proxy_address_input_field.set_disabled(false);
            user_token_input_field.set_disabled(false);
            listening_port_field.set_disabled(false);
            start_button.set_disabled(false);
        });
    })
}

pub fn on_register_btn_click(event: web_sys::MouseEvent) {
    gloo::console::info!("Receive register event: {:?}", event);
}

#[function_component(PpaassAgentUi)]
pub fn ppaass_agent_ui() -> Html {
    let initial_ui_info_state = use_state_eq(|| AgentUiInfo {
        user_token: "".to_string(),
        listening_port: "".to_string(),
        proxy_address: "".to_string(),
    });
    {
        let initial_ui_info_state = initial_ui_info_state.clone();
        spawn_local(async move {
            let ui_info = load_ui_info("load_ui_info").await;
            let ui_info: String = from_value(ui_info).unwrap();
            let ui_info = serde_json::from_str::<AgentUiInfo>(&ui_info).unwrap();
            initial_ui_info_state.set(ui_info);
        });
    }

    let global_style = StyleSource::try_from(include_str!("global.css")).unwrap();
    let user_name_field_ref = NodeRef::default();
    let proxy_address_field_ref = NodeRef::default();
    let listening_port_field_ref = NodeRef::default();
    let start_button_ref = NodeRef::default();

    let init_user_token_value = (*initial_ui_info_state.user_token).to_owned();
    let init_proxy_address_value = (*initial_ui_info_state.proxy_address).to_owned();
    let init_listening_port_value = (*initial_ui_info_state.listening_port).to_owned();

    gloo::console::info!("Initial user token: {}", &init_user_token_value);
    gloo::console::info!("Initial proxy address: {}", &init_proxy_address_value);
    gloo::console::info!("Initial listening port: {}", &init_listening_port_value);

    html! {
        <>
            <Global css={global_style} />
            <h1>{"Ppaass Agent"}</h1>
            <Container classes="input_field_panel">
                <InputField id="user_token" label={"User token:"}
                place_holder={"Enter user token"}
                hint="Register to get user token" input_ref={&user_name_field_ref} value={init_user_token_value}/>
                <InputField id="proxy_address" label={"Proxy address:"}
                place_holder={"Enter proxy address"}
                hint={format!("Default proxy address is: {}",init_proxy_address_value)} input_ref={&proxy_address_field_ref} value={init_proxy_address_value}/>
                <InputField id="listening_port" label={"Listening port:"}
                place_holder={"Enter listening port"}
                hint={format!("Default listening port is: {}", init_listening_port_value)} input_ref={&listening_port_field_ref} value={init_listening_port_value}/>
            </Container>
            <Container classes="button_panel">
                <Button id="register_button" label="Register" classes="button"
                on_click={on_register_btn_click} />
                <Button id="start_button" label="Start" classes="button" button_ref={&start_button_ref}
                on_click={generate_start_btn_callback(user_name_field_ref.clone(),proxy_address_field_ref.clone(), listening_port_field_ref.clone(), start_button_ref.clone())} />
                <Button id="stop_button" label="Stop" classes="button"
                on_click={generate_stop_btn_callback(user_name_field_ref,proxy_address_field_ref, listening_port_field_ref, start_button_ref)} />
            </Container>
            <Container classes="status_panel">
            {"Ready to start agent ..."}
            </Container>
        </>
    }
}
