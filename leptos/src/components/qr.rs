//! QrCode related components.

use leptos::prelude::*;
use leptos_qr_scanner::Scan;
use reactive_stores::Store;
use thaw::*;

use crate::css::styles;
use crate::storage::{GlobalState, GlobalStateStoreFields};

#[component]
pub fn QrScanner() -> impl IntoView {
    let scan = RwSignal::new(true);
    let multiple = RwSignal::new(true);

    let result = RwSignal::new(vec![]);

    view! {
        <Scan
            active=scan
            on_scan=move |a| {
                tracing::info!("Found: {}", &a);
                if !multiple.get_untracked() {
                    result.set(vec![a]);
                } else {
                    let vals = result.read_untracked();
                    if !vals.contains(&a) {
                        result.write().push(a)
                    }
                }
            }
            class=""
            video_class=styles::qrscanner
        />

        <Checkbox checked=scan label="Scan" />
        <Checkbox checked=multiple label="Multiple" />

        <p>Scan result: {result}</p>
    }
}
