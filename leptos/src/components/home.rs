use leptos::prelude::*;
use thaw::*;

#[component]
pub fn Home() -> impl IntoView {

    let email = RwSignal::new(String::from("i"));
    let password = RwSignal::new(String::from("p"));

    view! {
        <Flex vertical=true>
            <Input email placeholder="Email" />
            <Input password input_type=InputType::Password placeholder="OTP/API Key" />
            <Button>"Login"</Button>
        </Flex>
    }
}
