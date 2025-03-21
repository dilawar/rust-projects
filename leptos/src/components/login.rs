// use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
// use leptos_use::storage::use_local_storage;
use reactive_stores::Store;
use thaw::*;

use crate::css::styles;
use crate::storage::{GlobalState, GlobalStateStoreFields};

#[component]
pub fn Login() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let logged_in = state.is_logged_in();
    let email = state.email();
    let password = state.api_key();

    let login_clicked = move |_| {
        tracing::debug!("Login button is clicked. Email={}", email.get_untracked());
        tracing::info!("TODO: implement the login flow here");

        *logged_in.write() = true;
    };

    view! {
        <Flex vertical=true class=styles::login>
            <Input value=email placeholder="Email" />
            <Input value=password input_type=InputType::Password placeholder="OTP/API Key" />
            <Button on_click=login_clicked>"Login"</Button>
        </Flex>
    }
}

#[component]
pub fn AlreadyLoggedIn() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let logged_in = state.is_logged_in();
    let email = state.email();
    let password = state.api_key();

    let logout = move |_| {
        tracing::info!("Starting logout...");
        *email.write() = String::default();
        *password.write() = String::default();
        *logged_in.write() = false;
    };

    view! {
        <div>"Already logged in"</div>
        <Button on_click=logout>"Logout"</Button>
    }
}
