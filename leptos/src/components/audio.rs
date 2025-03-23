//! Audio Streamer component

use leptos::prelude::*;
use leptos_qr_scanner::Scan;
use reactive_stores::Store;
use thaw::*;

use crate::css::styles;
use crate::storage::{GlobalState, GlobalStateStoreFields};

#[component]
pub fn AudioStream() -> impl IntoView {
    let stream = RwSignal::new(true);

    let result = RwSignal::new(vec![]);

    view! {

        <Checkbox checked=stream label="Record" />
    }
}
