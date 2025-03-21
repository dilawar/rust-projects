use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use reactive_stores::Store;
use thaw::*;

use crate::css::styles;
use crate::components::{home, login};
use crate::storage::{GlobalState, GlobalStateStoreFields};

#[component]
pub fn App() -> impl IntoView {
    let theme = RwSignal::new(Theme::light());

    provide_context(Store::new(GlobalState::default()));

    // To check if we are already logged in.
    let store = expect_context::<Store<GlobalState>>();
    let logged_in = store.is_logged_in();

    view! {
        <Router>
            <h3>"Leptos App"</h3>
            <ConfigProvider theme>
                <nav class=styles::nav>
                    <a href="/">"Home"</a>
                    <a href="/form">"Form"</a>
                    <Show when=move || { logged_in.get() }>
                        <login::Logout />
                    </Show>
                </nav>
                <main>
                    <Routes fallback=|| "not found.">
                        <Route path=path!("/") view=home::Home />
                    </Routes>
                </main>
                <div class=styles::loading>
                    <AppSpinner />
                </div>
            </ConfigProvider>
        </Router>
    }
}

#[component]
fn AppSpinner() -> impl IntoView {
    let store = expect_context::<Store<GlobalState>>();
    let loading = store.loading();

    view! {
        <Show when=move || loading.get()>
            <Spinner />
        </Show>
    }
}
