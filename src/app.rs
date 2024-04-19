use std::{collections::VecDeque, rc::Rc};

use gloo::utils::format::JsValueSerdeExt;

use stylist::yew::Global;
use stylist::StyleSource;

use wasm_bindgen_futures::spawn_local;

use yew::prelude::*;

use ppaass_ui_common::{event::AgentEvent, payload::AgentConfigInfo};

use crate::{
    bo::ui_state::UiState,
    callbacks::{
        generate_agent_server_signal_callback, generate_agent_server_started_callback,
        generate_agent_server_stop_callback, AgentServerSignalCallbackParam,
        AgentServerStartedCallbackParam, AgentServerStopCallbackParam, StartBtnCallbackParam,
    },
    components::input_field::{InputField, InputFieldDataType},
};
use crate::{
    bo::{
        command::BackendCommand,
        ui_state::{StatusDetail, StatusLevel},
    },
    callbacks::{generate_start_btn_callback, generate_stop_btn_callback},
    components::container::Container,
};
use crate::{
    components::button::Button,
    components::network_info::NetworkInfo,
    wasm_binding::{invoke_tauri_without_arg, listen_tauri_event},
};

#[function_component(PpaassAgentUi)]
pub fn ppaass_agent_ui() -> Html {
    let global_style = StyleSource::try_from(include_str!("global.css")).unwrap();
    let user_token_field_ref = use_node_ref();
    let proxy_address_field_ref = use_node_ref();
    let listening_port_field_ref = use_node_ref();
    let start_button_ref = use_node_ref();
    let logging_textarea = use_node_ref();
    let ui_state = use_state(|| Option::<UiState>::None);
    let network_info_download_content_data = use_state(VecDeque::<String>::new);
    let network_info_upload_content_data = use_state(VecDeque::<String>::new);

    let start_btn_callback_param = StartBtnCallbackParam {
        user_token_input_ref: user_token_field_ref.clone(),
        proxy_address_field_ref: proxy_address_field_ref.clone(),
        listening_port_field_ref: listening_port_field_ref.clone(),
        ui_state: ui_state.clone(),
    };
    let agent_server_started_callback_param = AgentServerStartedCallbackParam {
        user_token_input_ref: user_token_field_ref.clone(),
        proxy_address_field_ref: proxy_address_field_ref.clone(),
        listening_port_field_ref: listening_port_field_ref.clone(),
        start_button_ref: start_button_ref.clone(),
        ui_state: ui_state.clone(),
    };
    let agent_server_stop_callback_param = AgentServerStopCallbackParam {
        user_token_input_ref: user_token_field_ref.clone(),
        proxy_address_field_ref: proxy_address_field_ref.clone(),
        listening_port_field_ref: listening_port_field_ref.clone(),
        start_button_ref: start_button_ref.clone(),
        ui_state: ui_state.clone(),
    };

    let agent_server_signal_callback_param = AgentServerSignalCallbackParam {
        ui_state: ui_state.clone(),
        network_info_download_content_data: network_info_download_content_data.clone(),
        network_info_upload_content_data: network_info_upload_content_data.clone(),
        logging_textarea: logging_textarea.clone(),
    };

    {
        let ui_state = ui_state.clone();
        use_effect(move || {
            //Do the logic when component initialize

            let agent_start_listener =
                generate_agent_server_started_callback(agent_server_started_callback_param);

            let agent_stop_listener =
                generate_agent_server_stop_callback(agent_server_stop_callback_param);

            let agent_signal_listener =
                generate_agent_server_signal_callback(agent_server_signal_callback_param);

            let agent_start_listener = Rc::new(agent_start_listener);
            let agent_stop_listener = Rc::new(agent_stop_listener);
            let agent_signal_listener = Rc::new(agent_signal_listener);
            {
                let agent_start_listener = agent_start_listener.clone();
                let agent_stop_listener = agent_stop_listener.clone();
                let agent_signal_listener = agent_signal_listener.clone();
                spawn_local(async move {
                    let _ = listen_tauri_event(
                        AgentEvent::Start.to_string().as_str(),
                        &agent_start_listener,
                    )
                    .await;
                    let _ = listen_tauri_event(
                        AgentEvent::Stop.to_string().as_str(),
                        &agent_stop_listener,
                    )
                    .await;
                    let _ = listen_tauri_event(
                        AgentEvent::Signal.to_string().as_str(),
                        &agent_signal_listener,
                    )
                    .await;
                });
            }

            if ui_state.is_none() {
                spawn_local(async move {
                    let config_info = match invoke_tauri_without_arg(
                        BackendCommand::LoadConfigInfo.to_string().as_str(),
                    )
                    .await
                    {
                        Ok(config_info) => config_info,
                        Err(e) => {
                            gloo::console::error!(
                            "Fail to load configuration inforamtion form backend because of error:",
                            e
                        );
                            return;
                        }
                    };

                    let config_info: AgentConfigInfo = config_info.into_serde().unwrap();
                    gloo::console::info!("Load config info:", format!("{config_info:?}"));
                    let new_ui_state = UiState {
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
                    ui_state.set(Some(new_ui_state));
                    gloo::console::info!("Agent initialized:", format!("{:?}", *ui_state));
                });
            }

            move || {
                // Do the logic when component destroy
                // The listener must drop here otherwise it will cause multiple listener registered.
                // When listener dropped the event will not be listened.
                drop(agent_start_listener);
                drop(agent_stop_listener);
                drop(agent_signal_listener);
            }
        });
    }

    match *ui_state {
        None => {
            html! {
                <div class="loading_message">{"Loading ..."}</div>
            }
        }
        Some(ref ui_state_inner) => {
            let status_detail = ui_state_inner.status_detail.clone();
            let user_token = ui_state_inner.user_token.clone();
            let proxy_address = ui_state_inner.proxy_address.clone();
            let listening_port = ui_state_inner.listening_port.clone();

            let upload_network_info = format!(
                "↑↑↑ Total: {:.2} MB; Avg {:.2} MB/S",
                ui_state_inner.network_detail.upload_bytes_amount as f64 / (1024 * 1024) as f64,
                ui_state_inner.network_detail.upload_mb_per_second
            );
            let download_network_info = format!(
                "↓↓↓ Total: {:.2} MB; Avg: {:.2} MB/S",
                ui_state_inner.network_detail.download_bytes_amount as f64 / (1024 * 1024) as f64,
                ui_state_inner.network_detail.download_mb_per_second
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
                            data_type={InputFieldDataType::Addresses}
                            hint={"Proxy addresses are seperate with \";\""} input_ref={&proxy_address_field_ref} value={proxy_address}/>
                            <InputField id="listening_port" label={"Listening port:"}
                            place_holder={"Enter listening port"}
                            data_type={InputFieldDataType::Number{min: 1025, max: 65535}}
                            hint={"Listening port should between 1025~65536"} input_ref={&listening_port_field_ref} value={listening_port}/>
                        </Container>
                        <Container classes="button_panel">
                            <Button id="start_button" label="Start" classes="button" button_ref={&start_button_ref}
                            on_click={generate_start_btn_callback(start_btn_callback_param)} />
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
                            <NetworkInfo download_content_data={(*network_info_download_content_data).clone()} upload_content_data={(*network_info_upload_content_data).clone()}></NetworkInfo>
                        </Container>
                        <Container classes="logging">
                            <label for="logging_textarea">{"Logging information"}</label>
                            <textarea id="logging_textarea" ref={logging_textarea} readonly={true}></textarea>
                        </Container>
                    </div>
                </>
            }
        }
    }
}
