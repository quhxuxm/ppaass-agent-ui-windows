use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use stylist::{yew::Global, StyleSource};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{platform::pinned::oneshot::channel, prelude::*};

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

#[derive(Serialize, Deserialize, Debug)]
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
) -> Callback<web_sys::MouseEvent> {
    Callback::from(move |event: web_sys::MouseEvent| {
        let user_token_input_field = user_token_input_ref.cast::<HtmlInputElement>().unwrap();
        let proxy_address_input_field = proxy_address_field_ref.cast::<HtmlInputElement>().unwrap();
        let listening_port_field = listening_port_field_ref.cast::<HtmlInputElement>().unwrap();
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
        });
    })
}

pub fn on_stop_btn_click(event: web_sys::MouseEvent) {
    gloo::console::info!("Receive stop event: {:?}", event);
    spawn_local(async move {
        invoke_without_arg("stop_vpn").await;
    });
}

pub fn on_register_btn_click(event: web_sys::MouseEvent) {
    gloo::console::info!("Receive register event: {:?}", event);
}

#[function_component(PpaassAgentUi)]
pub fn ppaass_agent_ui() -> Html {
    let ui_info_state = use_state(|| AgentUiInfo {
        user_token: "".to_string(),
        listening_port: "".to_string(),
        proxy_address: "".to_string(),
    });
    {
        let ui_info_state = ui_info_state.clone();
        spawn_local(async move {
            let ui_info = load_ui_info("load_ui_info").await;
            let ui_info: String = from_value(ui_info).unwrap();
            let ui_info = serde_json::from_str::<AgentUiInfo>(&ui_info).unwrap();
            gloo::console::info!(format!("Receive ui state from backend (1): {ui_info:?}"));
            ui_info_state.set(ui_info);
        });
    }

    gloo::console::info!(format!(
        "Receive ui state from backend(2): {:?}",
        *ui_info_state
    ));
    // let ui_info: AgentUiInfo = ui_info_js_val.try_into().unwrap();
    // gloo::console::info!("Initial UI info: {}", format!("{ui_info:?}"));
    let global_style = StyleSource::try_from(include_str!("global.css")).unwrap();
    let user_name_field_ref = NodeRef::default();
    let proxy_address_field_ref = NodeRef::default();
    let listening_port_field_ref = NodeRef::default();

    let init_user_token_value = (*ui_info_state.user_token).to_owned();
    let init_proxy_address_value = (*ui_info_state.proxy_address).to_owned();
    let init_listening_port_value = (*ui_info_state.listening_port).to_owned();

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
                hint="Default proxy address is 127.0.0.1" input_ref={&proxy_address_field_ref} value={init_proxy_address_value}/>
                <InputField id="listening_port" label={"Listening port:"}
                place_holder={"Enter listening port"}
                hint="Default listening port is 10080" input_ref={&listening_port_field_ref} value={init_listening_port_value}/>
            </Container>
            <Container classes="button_panel">
                <Button id="register_button" label="Register" classes="button"
                on_click={on_register_btn_click} />
                <Button id="start_button" label="Start"  classes="button"
                on_click={generate_start_btn_callback(user_name_field_ref,proxy_address_field_ref, listening_port_field_ref)} />
                <Button id="stop_button" label="Stop" classes="button"
                on_click={on_stop_btn_click} />
            </Container>
            <Container classes="status_pannel">
            {"Ready to start agent ..."}
            </Container>
        </>
    }
}
