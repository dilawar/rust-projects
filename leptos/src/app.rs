use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;


use leptos::prelude::*;
use thaw::*;

use crate::components::login;
use crate::components::home;
use crate::css::styles;


#[component]
pub fn App() -> impl IntoView {

    let theme = RwSignal::new(Theme::light());

    view! {
        <Router>
            <h3>"Rusty App"</h3>
            <ConfigProvider theme>
                <nav class=styles::nav>
                    <a href="/">"Home"</a>
                    <a href="/ehr/form">"EHR Form"</a>
                </nav>
                <main>
                    <Routes fallback=|| "not found.">
                        <Route path=path!("/") view=home::Home />
                        <Route path=path!("/login") view=login::Login />
                        <Route path=path!("/ehr/form") view=home::Home />
                    </Routes>
                </main>
            </ConfigProvider>
        </Router>
    }
}
