use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use leptos_use::storage::use_local_storage;
use codee::string::JsonSerdeCodec;

use crate::components::home;
use crate::storage::AppState;

stylance::import_crate_style!( #[allow(dead_code)] app_style, "src/app.module.scss");

#[component]
pub fn App() -> impl IntoView {

    let (storage_email, _set_storage_email) = signal("email".to_string());
    let (_state, _set_state, _reset) = use_local_storage::<AppState, JsonSerdeCodec>(storage_email);


    view! {
        <Router>
            <h1>"Leptos App"</h1>
            <nav class=app_style::nav>
                <a href="/">"Home"</a>
                <a href="/ehr/form">"EHR Form"</a>
            </nav>
            <main>
                <Routes fallback=|| "not found.">
                    <Route path=path!("/") view=home::Home />
                    <Route path=path!("/ehr/form") view=home::Home />
                </Routes>
            </main>
        </Router>
    }
}
