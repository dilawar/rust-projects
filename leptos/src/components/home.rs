use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    let email = RwSignal::new("".to_string());
    let password = RwSignal::new("".to_string());

    view! {
        <input type="text" bind:value=email />
        <input type="password" bind:value=password />
        <button>Login</button>
    }
}
