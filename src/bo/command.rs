use derive_more::Display;
#[derive(Debug, Display)]
pub enum UiBackendCommand {
    #[display(fmt = "start_agent_server")]
    StartAgentServer,
    #[display(fmt = "stop_agent_server")]
    StopAgentServer,
    #[display(fmt = "load_agent_server_configuration")]
    LoadAgentServerConifugration,
}
