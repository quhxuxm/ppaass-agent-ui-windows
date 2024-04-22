use derive_more::Display;

#[derive(Debug, PartialEq, Eq, Clone, Default, Display)]
pub enum StatusLevel {
    #[display(fmt = "error")]
    Error,
    #[display(fmt = "info")]
    #[default]
    Info,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct StatusDetail {
    pub text: String,
    pub level: StatusLevel,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct NetworkDetail {
    pub upload_mb_amount: u64,
    pub upload_mb_per_second: f64,
    pub download_mb_amount: u64,
    pub download_mb_per_second: f64,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct UiState {
    pub user_token: String,
    pub proxy_address: String,
    pub listening_port: String,
    pub status_detail: StatusDetail,
    pub network_detail: NetworkDetail,
}
