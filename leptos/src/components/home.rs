use leptos::prelude::*;

use crate::components::login::{AlreadyLoggedIn, Login};

#[component]
pub fn Home() -> impl IntoView {

    let is_logged_in = RwSignal::new(false);

    view! {
        <Show when=move || !is_logged_in.get() fallback=|| view! { <AlreadyLoggedIn /> }>
            <Login />
        </Show>
    }
}
