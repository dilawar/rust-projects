//! Various storage

#![allow(dead_code)]

use std::collections::HashMap;
use reactive_stores::Store;

// Local store to store data before we 
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Default)]
pub(crate) struct EhrSection1 {
    pub sample_code: String,
    pub patient_name: String,
}

// Local store to store data before we 
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Default)]
pub(crate) struct KeyVal(pub HashMap<String, String>);

impl EhrSection1 {
    /// Key to use in local storage
    pub const KEY: &'static str = "ehr-section-1";
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
    /// Loading (show spinner when true)
    pub loading: bool,
}
