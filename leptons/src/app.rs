use leptos::prelude::*;
use leptos_router::components::{Router, Route, Routes};
use leptos_router::path;

use crate::components::home;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav></nav>
            <main>
                <Routes fallback=|| "not found.">
                    <Route path=path!("/") view=home::Home />
                </Routes>
            </main>
        </Router>
    }
}

