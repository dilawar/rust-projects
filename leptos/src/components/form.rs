//! EHR form

use leptos::prelude::*;
use leptos_qr_scanner::Scan;
use reactive_stores::Store;
use thaw::*;

use crate::css::styles;
use crate::storage::{GlobalState, GlobalStateStoreFields};

#[component]
pub fn Form() -> impl IntoView {
    view! {
        <Space vertical=true>
            <h5>"EHR Form"</h5>
            <Input placeholder="Sample code"></Input>
            <Input placeholder="Patient Name"></Input>
        </Space>
    }
}
