use leptos::prelude::*;
use leptos::logging::log;
use thaw::*;

#[component]
pub fn Home() -> impl IntoView {

    let email = RwSignal::new(String::from(""));
    let password = RwSignal::new(String::from(""));

    let login_clicked = move |_| {
        log!("Login button is clicked");
        log!("Email={} and password={}", email.get(), password.get());
    };

    view! {
        <Flex vertical=true>
            <Input value=email placeholder="Email" />
            <Input value=password input_type=InputType::Password placeholder="OTP/API Key" />
            <Button on_click=login_clicked>"Login"</Button>
        </Flex>
    }
}
