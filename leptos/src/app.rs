use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use reactive_stores::Store;
use thaw::*;

use crate::components::{home};
use crate::storage::{GlobalState, GlobalStateStoreFields};

#[component]
pub fn App() -> impl IntoView {
    let theme = RwSignal::new(Theme::light());

    provide_context(Store::new(GlobalState::default()));

    let store = expect_context::<Store<GlobalState>>();
    let logged_in = store.is_logged_in();

    view! {
        <Router>
            <h3>"Leptos App"</h3>
            <ConfigProvider theme>
                <nav class=crate::css::styles::nav>
                    <a href="/">"Home"</a>
                    <a href="/form">"Form"</a>
                    <Show when=move ||  { logged_in.get() } >
                        <a href="">"Logout"</a>
                    </Show>
                </nav>
                <main>
                    <Routes fallback=|| "not found.">
                        <Route path=path!("/") view=home::Home />
                    </Routes>
                </main>
            </ConfigProvider>
        </Router>
    }
}
