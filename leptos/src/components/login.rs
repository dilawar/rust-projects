use leptos::logging::log;

use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use thaw::*;

use crate::css::styles;
use crate::storage::LoginState;

#[component]
pub fn Login() -> impl IntoView {
    let (storage_key, _) = signal(LoginState::KEY.to_string());
    let (state, set_state, _) = use_local_storage::<LoginState, JsonSerdeCodec>(storage_key);

    let email_init = state.get().email;
    let api_key_init = state.get().api_key;

    let email = RwSignal::new(email_init);
    let password = RwSignal::new(api_key_init);

    let login_clicked = move |_| {
        log!("Login button is clicked");
        set_state.update(|s| s.email = email.get());
        set_state.update(|s| s.api_key = password.get());
        set_state.update(|s| s.logged_in = true);
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
    view! { <div></div> }
}
