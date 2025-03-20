use leptos::logging::log;

use leptos::prelude::*;
use thaw::*;

use crate::css::styles;

#[component]
pub fn Login() -> impl IntoView {
    let email = RwSignal::new(String::from(""));
    let password = RwSignal::new(String::from(""));

    let login_clicked = move |_| {
        log!("Login button is clicked");
        log!("Email={} and password={}", email.get(), password.get());
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
    view! {
        <Flex>
            <Text>"Already logged in!"</Text>
        </Flex>
    }
}
