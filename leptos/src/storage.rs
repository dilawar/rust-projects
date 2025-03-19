//! Storage

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Default)]
pub struct AppState {
    pub email: String,
}
