use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use stylist::{yew::Global, StyleSource};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::{button::Button, container::Container, input_field::InputField};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name ="invoke")]
    async fn invoke_with_arg(cmd: &str, args: JsValue);

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name ="invoke")]
    async fn invoke_without_arg(cmd: &str);
}

#[derive(Serialize, Deserialize)]
pub struct AgentUiInfoArg {
    info: AgentUiInfo,
}

#[derive(Serialize, Deserialize)]
pub struct AgentUiInfo {
    user_token: String,
    proxy_address: String,
}

pub fn generate_start_btn_callback(
    user_token_input_ref: NodeRef,
    proxy_address_field_ref: NodeRef,
) -> Callback<web_sys::MouseEvent> {
    Callback::from(move |event: web_sys::MouseEvent| {
        let user_token_input_field = user_token_input_ref.cast::<HtmlInputElement>().unwrap();
        let proxy_address_input_field = proxy_address_field_ref.cast::<HtmlInputElement>().unwrap();
        gloo::console::info!("Receive start event: {:?}", event);
        gloo::console::info!("The user token: {}", user_token_input_field.value());
        gloo::console::info!("The proxy address: {}", proxy_address_input_field.value());
        let agent_ui_info_arg = AgentUiInfoArg {
            info: AgentUiInfo {
                user_token: user_token_input_field.value(),
                proxy_address: proxy_address_input_field.value(),
            },
        };

        spawn_local(async move {
            let args = to_value(&agent_ui_info_arg).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            invoke_with_arg("start_vpn", args).await;
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
    let global_style = StyleSource::try_from(include_str!("global.css")).unwrap();
    let user_name_field_ref = NodeRef::default();
    let proxy_address_field_ref = NodeRef::default();
    html! {
        <>
            <Global css={global_style} />
            <h1>{"Ppaass Agent"}</h1>
            <Container classes="input_field_pannel">
                <InputField id="user_token" label={"User token:"}
                place_holder={"Enter user token"}
                hint="Register to get user token" input_ref={&user_name_field_ref}/>
                <InputField id="proxy_address" label={"Proxy address:"}
                place_holder={"Enter proxy address"}
                hint="Defual proxy address is 127.0.0.1" input_ref={&proxy_address_field_ref}/>
            </Container>
            <Container classes="button_pannel">
                <Button id="register_button" label="Register" classes="button"
                on_click={on_register_btn_click} />
                <Button id="start_button" label="Start"  classes="button"
                on_click={generate_start_btn_callback(user_name_field_ref,proxy_address_field_ref)} />
                <Button id="stop_button" label="Stop" classes="button"
                on_click={on_stop_btn_click} />
            </Container>
            <Container classes="status_pannel">
            {"Ready to start agent ..."}
            </Container>
        </>
    }
}
