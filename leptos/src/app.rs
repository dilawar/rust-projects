use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use reactive_stores::Store;
use thaw::*;

use crate::components::*;
use crate::css::styles;
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
            <h3>"Dognosis Portal"</h3>
            <ConfigProvider theme>
                <nav class=styles::nav>
                    <a href="/">"Home"</a>
                    <a href="/form">"Form"</a>
                    <a href="/qr">"QR Scanner"</a>
                    <a href="/audio">"Audio Rec"</a>
                    <Show when=move || { logged_in.get() }>
                        <login::Logout />
                    </Show>
                </nav>
                <main>
                    <Routes fallback=|| "Page not found.">
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/form") view=Form />
                        <Route path=path!("/qr") view=QrScanner />
                        <Route path=path!("/audio") view=AudioStream />
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
        // Show spinner when global `loading` is set to true.
        <Show when=move || loading.get()>
            <Spinner />
        </Show>
    }
}
