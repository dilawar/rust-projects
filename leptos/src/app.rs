use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use reactive_stores::Store;
use thaw::*;

use crate::components::{home, login};
use crate::storage::GlobalState;

#[component]
pub fn App() -> impl IntoView {
    let theme = RwSignal::new(Theme::light());

    provide_context(Store::new(GlobalState::default()));

    view! {
        <Router>
            <h3>"Rusty App"</h3>
            <ConfigProvider theme>
                <nav class=crate::css::styles::nav>
                    <a href="/">"Home"</a>
                    <a href="/form">"Form"</a>
                    <a href="/login">"Login"</a>
                </nav>
                <main>
                    <Routes fallback=|| "not found.">
                        <Route path=path!("/") view=home::Home />
                        <Route path=path!("/login") view=login::Login />
                    </Routes>
                </main>
            </ConfigProvider>
        </Router>
    }
}
