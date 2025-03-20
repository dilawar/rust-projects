use leptos::prelude::*;

use crate::components::login::{AlreadyLoggedIn, Login};

#[component]
pub fn Home() -> impl IntoView {

    let is_logged_in = RwSignal::new(false);

    view! {
        <h1>"Login"</h1>
        {move || match is_logged_in.get() {
            true => view! { 
                <Login /> 
            }.into_view(),
            _ => view! { 
                <AlreadyLoggedIn />
            }.into_view(),
        }}
    }
}
