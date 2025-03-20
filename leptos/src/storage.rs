//! Storage

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Default)]
pub struct LoginState {
    pub email: String,
    pub api_key: String,
    pub logged_in: bool,
}

impl LoginState {
    // Key to use in local storage
    pub const KEY: &'static str = "login-state";
}
