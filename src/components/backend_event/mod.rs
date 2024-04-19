use gloo::utils::format::JsValueSerdeExt;
use ppaass_ui_common::{
    event::AgentEvent,
    payload::{AgentServerSignalPayload, AgentServerSignalType},
};
use wasm_bindgen::{closure::Closure, JsValue};
use yew::{function_component, html, use_effect, Callback, Html, Properties};

use crate::{bo::BackendEventWrapper, wasm_binding::listen_tauri_event};

#[derive(Properties, PartialEq)]
pub struct BackendEventProps {
    network_state_callback: Callback<String>,
    logging_state_callback: Callback<String>,
    agent_start_callback: Callback<String>,
    agent_stop_callback: Callback<String>,
}

fn generate_backend_event_callback() -> Closure<dyn FnMut(JsValue)> {
    Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
        let backend_event: BackendEventWrapper<AgentServerSignalPayload> =
            event.into_serde().unwrap();
        let agent_server_signal = backend_event.payload;
        match agent_server_signal.signal_type {
            AgentServerSignalType::Info => todo!(),
            AgentServerSignalType::Error => todo!(),
            AgentServerSignalType::NetworkInfo {
                upload_bytes_amount,
                upload_mb_per_second,
                download_bytes_amount,
                download_mb_per_second,
            } => todo!(),
        }
    })
}

#[function_component(BackendEvent)]
pub fn backend_event() -> Html {
    use_effect(|| {
        let agent_signal_listener = generate_backend_event_callback();
        listen_tauri_event(
            AgentEvent::Signal.to_string().as_str(),
            &agent_signal_listener,
        );
        || drop(agent_signal_listener)
    });
    html!()
}
