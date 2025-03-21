//! Various storage

#![allow(dead_code)]

use reactive_stores::Store;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Default)]
pub(crate) struct LocalStorage {
    pub trainer: String,
}

impl LocalStorage {
    /// Key to use in local storage
    pub const KEY: &'static str = "login-state";
}

/// Global state to be shared across components.
#[derive(Clone, Debug, Default, Store)]
pub struct GlobalState {
    /// Current user email.
    pub email: String,
    /// API Key
    pub api_key: String,
    /// Is user logged in
    pub is_logged_in: bool,
}
