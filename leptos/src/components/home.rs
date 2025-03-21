use leptos::prelude::*;
use reactive_stores::Store;

use crate::components::login::{Login};
use crate::storage::{GlobalState, GlobalStateStoreFields};

#[component]
pub fn Home() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();

    let logged_in = state.is_logged_in();
    tracing::info!("Already logged in? {}", logged_in.get_untracked());

    view! {
        <Show when=move || !logged_in.get()>
            <Login />
        </Show>
    }
}
