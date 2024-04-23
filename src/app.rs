use gloo::utils::format::JsValueSerdeExt;

use ppaass_agent_ui_model::{
    model::{UiModelAgentServerConfiguration, UiModelStatusBarDetail, UiModelStatusBarDetailType},
    AGENT_SERVER_EVENT,
};
use wasm_bindgen_futures::spawn_local;

use yew::prelude::*;

use crate::components::network_bar::NetworkBar;
use crate::components::network_chart::NetworkChart;
use crate::{
    bo::UiBackendCommand,
    callbacks::{
        generate_backend_event_listener_callback, generate_start_agent_server_btn_callback,
        generate_stop_agent_server_btn_callback, UiCallbackParamBackendEventListener,
        UiCallbackParamStartAgentServer,
    },
    components::input_field::{InputField, InputFieldDataType},
};
use crate::{bo::UiStateMainPage, components::container::Container};
use crate::{
    components::button::Button,
    wasm_binding::{invoke_tauri_without_arg, listen_tauri_event},
};

#[function_component(AgentServerConfigurationUi)]
pub fn agent_server_config_ui() -> Html {
    // The ui elements
    let user_token_input_ref = use_node_ref();
    let proxy_address_input_ref = use_node_ref();
    let listening_port_input_ref = use_node_ref();
    let start_button_ref = use_node_ref();
    let stop_button_ref = use_node_ref();
    let logging_textarea_ref = use_node_ref();

    // The ui state of the main page
    let main_page_ui_state = use_state(|| UiStateMainPage {
        status_bar_detail: UiModelStatusBarDetail {
            level: UiModelStatusBarDetailType::Info,
            text: "Ready to start agent server".to_string(),
        },
        ..Default::default()
    });

    let start_agent_server_btn_callback =
        generate_start_agent_server_btn_callback(UiCallbackParamStartAgentServer {
            user_token_input_ref: user_token_input_ref.clone(),
            listening_port_input_ref: listening_port_input_ref.clone(),
            proxy_address_input_ref: proxy_address_input_ref.clone(),
        });
    let stop_agent_server_btn_callback = generate_stop_agent_server_btn_callback();
    {
        let user_token_input_ref = user_token_input_ref.clone();
        let proxy_address_input_ref = proxy_address_input_ref.clone();
        let listening_port_input_ref = listening_port_input_ref.clone();
        let logging_textarea_ref = logging_textarea_ref.clone();
        let start_button_ref = start_button_ref.clone();
        let stop_button_ref = stop_button_ref.clone();

        let main_page_ui_state = main_page_ui_state.clone();

        use_effect(move || {
            //Do the logic when component initialize
            let ui_callback_param_start_agent_server = UiCallbackParamBackendEventListener {
                user_token_input_ref,
                listening_port_input_ref,
                proxy_address_input_ref,
                start_button_ref,
                stop_button_ref,
                logging_textarea_ref,
                main_page_ui_state: main_page_ui_state.clone(),
            };
            let backend_event_listener =
                generate_backend_event_listener_callback(ui_callback_param_start_agent_server);

            listen_tauri_event(AGENT_SERVER_EVENT, &backend_event_listener);

            if main_page_ui_state.configuration.is_none() {
                spawn_local(async move {
                    let configuration_js_value = match invoke_tauri_without_arg(
                        UiBackendCommand::LoadAgentServerConifugration.name(),
                    )
                    .await
                    {
                        Ok(configuration) => configuration,
                        Err(e) => {
                            gloo::console::error!("Fail to load configuration inforamtion form backend because of error:", e);
                            return;
                        }
                    };
                    let ui_model_agent_server_configuration: UiModelAgentServerConfiguration =
                        configuration_js_value.into_serde().unwrap();
                    gloo::console::info!(
                        "Load agent server configuraiton:",
                        format!("{ui_model_agent_server_configuration:?}")
                    );
                    let new_main_page_ui_state = UiStateMainPage {
                        configuration: Some(ui_model_agent_server_configuration),
                        ..(*main_page_ui_state).clone()
                    };
                    main_page_ui_state.set(new_main_page_ui_state);
                });
            }

            move || {
                // Do the logic when component destroy
                // The listener must drop here otherwise it will cause multiple listener registered.
                // When listener dropped the event will not be listened.
                drop(backend_event_listener);
            }
        });
    }

    match main_page_ui_state.configuration {
        None => {
            html! {
                <div class="loading_message">{"Loading ..."}</div>
            }
        }
        Some(ref ui_state_agent_server_config) => {
            let user_token = ui_state_agent_server_config.user_token.clone();
            let proxy_address = ui_state_agent_server_config.proxy_address.clone();
            let listening_port = ui_state_agent_server_config.listening_port;

            html! {
                <>
                    <div class="left_panel">
                        <Container classes="input_field_panel">
                            <InputField id="user_token" label={"User token:"}
                            place_holder={"Enter user token"}
                            hint="Register to get user token" input_ref={user_token_input_ref} value={user_token}/>
                            <InputField id="proxy_address" label={"Proxy address:"}
                            place_holder={"Enter proxy address"}
                            data_type={InputFieldDataType::Addresses}
                            hint={"Proxy addresses are seperate with \";\""} input_ref={proxy_address_input_ref} value={proxy_address.join(";")}/>
                            <InputField id="listening_port" label={"Listening port:"}
                            place_holder={"Enter listening port"}
                            data_type={InputFieldDataType::Number{min: 1025, max: 65535}}
                            hint={"Listening port should between 1025~65536"} input_ref={listening_port_input_ref} value={listening_port.to_string()}/>
                        </Container>
                        <Container classes="button_panel">
                            <Button id="start_button" label="Start" classes="button" button_ref={start_button_ref}
                            on_click={start_agent_server_btn_callback} />
                            <Button id="stop_button" label="Stop" classes="button"
                            button_ref={stop_button_ref}
                            on_click={stop_agent_server_btn_callback} />
                        </Container>
                        <Container classes="network_panel">
                            <NetworkBar upload_mb_amount={main_page_ui_state.network_detail.upload_mb_amount} upload_mb_per_second={main_page_ui_state.network_detail.upload_mb_per_second} download_mb_amount={main_page_ui_state.network_detail.download_mb_amount} download_mb_per_second={main_page_ui_state.network_detail.download_mb_per_second} >
                            </NetworkBar>
                        </Container>
                        <Container classes="status_panel">
                            <span class={["log".to_string(), main_page_ui_state.status_bar_detail.level.name().to_owned()]}>
                                {&*main_page_ui_state.status_bar_detail.text}
                            </span>
                        </Container>

                    </div>

                    <div class="right_panel">
                        <Container classes="network_status">
                            <label>{"Network status"}</label>
                            <NetworkChart download_content_data={main_page_ui_state.network_chart_download_mb_per_second.clone()} upload_content_data={main_page_ui_state.network_chart_upload_mb_per_second.clone()}>
                            </NetworkChart>
                        </Container>
                        <Container classes="logging">
                            <label for="logging_textarea">{"Logging information"}</label>
                            <textarea id="logging_textarea" ref={logging_textarea_ref} readonly={true}>
                            </textarea>
                        </Container>
                    </div>
                </>
            }
        }
    }
}
