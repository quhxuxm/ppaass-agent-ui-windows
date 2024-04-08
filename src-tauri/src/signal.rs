use ppaass_agent::server::AgentServerSignal;
use ppaass_ui_common::{
    event::AgentEvent,
    payload::{AgentServerSignalLevel, AgentServerSignalPayload},
};

use tauri::Window;

pub fn dispatch_signal(signal: AgentServerSignal, window: &Window) {
    let signal_event_name = AgentEvent::Signal.to_string();
    let signal_event_name = signal_event_name.as_str();
    match signal {
        AgentServerSignal::FailToListen(message) => {
            window
                .emit(
                    signal_event_name,
                    AgentServerSignalPayload {
                        client_socket_address: None,
                        level: AgentServerSignalLevel::Error,
                        message,
                    },
                )
                .unwrap();
        }
        AgentServerSignal::SuccessToListen(message) => {
            window
                .emit(
                    signal_event_name,
                    AgentServerSignalPayload {
                        client_socket_address: None,
                        level: AgentServerSignalLevel::Info,
                        message,
                    },
                )
                .unwrap();
        }
        AgentServerSignal::ClientConnectionAcceptSuccess {
            client_socket_address,
            message,
        } => {
            window
                .emit(
                    signal_event_name,
                    AgentServerSignalPayload {
                        client_socket_address: Some(client_socket_address),
                        level: AgentServerSignalLevel::Info,
                        message,
                    },
                )
                .unwrap();
        }
        AgentServerSignal::ClientConnectionAcceptFail(message) => {
            window
                .emit(
                    signal_event_name,
                    AgentServerSignalPayload {
                        client_socket_address: None,
                        level: AgentServerSignalLevel::Error,
                        message,
                    },
                )
                .unwrap();
        }
        AgentServerSignal::ClientConnectionBeforeRelayFail {
            client_socket_address,
            message,
        } => {
            window
                .emit(
                    signal_event_name,
                    AgentServerSignalPayload {
                        client_socket_address: Some(client_socket_address),
                        level: AgentServerSignalLevel::Error,
                        message,
                    },
                )
                .unwrap();
        }
        AgentServerSignal::ClientConnectionReadProxyConnectionWriteClose {
            client_socket_address,
            message,
        } => {
            window
                .emit(
                    signal_event_name,
                    AgentServerSignalPayload {
                        client_socket_address: Some(client_socket_address),
                        level: AgentServerSignalLevel::Info,
                        message,
                    },
                )
                .unwrap();
        }
        AgentServerSignal::ClientConnectionWriteProxyConnectionReadClose {
            client_socket_address,
            message,
        } => {
            window
                .emit(
                    signal_event_name,
                    AgentServerSignalPayload {
                        client_socket_address: Some(client_socket_address),
                        level: AgentServerSignalLevel::Info,
                        message,
                    },
                )
                .unwrap();
        }
        AgentServerSignal::ClientConnectionTransportCreateFail {
            client_socket_address,
            message,
            ..
        } => {
            window
                .emit(
                    signal_event_name,
                    AgentServerSignalPayload {
                        client_socket_address: Some(client_socket_address),
                        level: AgentServerSignalLevel::Error,
                        message,
                    },
                )
                .unwrap();
        }
        AgentServerSignal::ClientConnectionTransportCreateSuccess {
            client_socket_address,
            message,
            ..
        } => {
            window
                .emit(
                    signal_event_name,
                    AgentServerSignalPayload {
                        client_socket_address: Some(client_socket_address),
                        level: AgentServerSignalLevel::Info,
                        message,
                    },
                )
                .unwrap();
        }
        AgentServerSignal::ClientConnectionTransportCreateProxyConnectionFail {
            client_socket_address,
            message,
            ..
        } => {
            window
                .emit(
                    signal_event_name,
                    AgentServerSignalPayload {
                        client_socket_address: Some(client_socket_address),
                        level: AgentServerSignalLevel::Error,
                        message,
                    },
                )
                .unwrap();
        }
        AgentServerSignal::ClientConnectionTransportCreateProxyConnectionSuccess {
            client_socket_address,
            message,
            ..
        } => {
            window
                .emit(
                    signal_event_name,
                    AgentServerSignalPayload {
                        client_socket_address: Some(client_socket_address),
                        level: AgentServerSignalLevel::Info,
                        message,
                    },
                )
                .unwrap();
        }
    }
}
