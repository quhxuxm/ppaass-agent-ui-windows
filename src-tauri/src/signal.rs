use ppaass_agent::server::AgentServerSignal;
use ppaass_ui_common::{
    event::AgentEvent,
    payload::{AgentServerSignalPayload, AgentServerSignalType},
};

use tauri::Window;

pub fn dispatch_signal(signal: AgentServerSignal, window: &Window) {
    let signal_event_name = AgentEvent::Signal.to_string();
    let signal_event_name = signal_event_name.as_str();
    match signal {
        AgentServerSignal::NetworkInfo {
            upload_bytes_amount,
            upload_mb_per_second,
            download_bytes_amount,
            download_mb_per_second,
        } => {
            window
                .emit(
                    signal_event_name,
                    AgentServerSignalPayload {
                        client_socket_address: None,
                        signal_type: AgentServerSignalType::NetworkInfo {
                            upload_bytes_amount,
                            upload_mb_per_second,
                            download_bytes_amount,
                            download_mb_per_second,
                        },
                        message: None,
                    },
                )
                .unwrap();
        }
        AgentServerSignal::FailToListen(message) => {
            window
                .emit(
                    signal_event_name,
                    AgentServerSignalPayload {
                        client_socket_address: None,
                        signal_type: AgentServerSignalType::Error,
                        message: Some(message),
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
                        signal_type: AgentServerSignalType::Info,
                        message: Some(message),
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
                        signal_type: AgentServerSignalType::Info,
                        message: Some(message),
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
                        signal_type: AgentServerSignalType::Error,
                        message: Some(message),
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
                        signal_type: AgentServerSignalType::Error,
                        message: Some(message),
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
                        signal_type: AgentServerSignalType::Info,
                        message: Some(message),
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
                        signal_type: AgentServerSignalType::Info,
                        message: Some(message),
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
                        signal_type: AgentServerSignalType::Error,
                        message: Some(message),
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
                        signal_type: AgentServerSignalType::Info,
                        message: Some(message),
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
                        signal_type: AgentServerSignalType::Error,
                        message: Some(message),
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
                        signal_type: AgentServerSignalType::Info,
                        message: Some(message),
                    },
                )
                .unwrap();
        }
    }
}
