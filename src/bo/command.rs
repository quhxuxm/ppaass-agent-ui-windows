use derive_more::Display;
#[derive(Debug, Display)]
pub enum BackendCommand {
    #[display(fmt = "start_vpn")]
    AgentStart,
    #[display(fmt = "stop_vpn")]
    AgentStop,
    #[display(fmt = "load_ui_info")]
    LoadConfigInfo,
}
