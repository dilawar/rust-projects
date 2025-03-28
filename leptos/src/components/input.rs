//! Input component.

use crate::storage::KeyVal;
use itertools::Itertools;
use leptos::prelude::*;
use thaw::*;

#[component]
pub fn InputWithLabel(
    key: String,
    state: Signal<KeyVal>,
    set_state: WriteSignal<KeyVal>,
) -> impl IntoView {
    let label = key.split("_").join(" ");
    let key1 = key.to_string();

    view! {
        <Flex>
            <Label>{label}</Label>
            // Could not get thaw::Input to change when value in parent changes.
            <input
                prop:value=move || {
                    state.get().0.get(&key1).map(|x| x.to_string()).unwrap_or_default()
                }
                on:input=move |e| {
                    set_state
                        .update(|s| {
                            s.0.insert(key.to_string(), event_target_value(&e));
                        })
                }
            />
        </Flex>
    }
}

#[component]
pub fn SelectWithLabel(
    key: String,
    options: Vec<String>,
    state: Signal<KeyVal>,
    set_state: WriteSignal<KeyVal>,
) -> impl IntoView {

    let label = key.split("_").join(" ");
    let key1 = key.to_string();

    view! {
        <Flex>
            <Label>{label}</Label>
            <select
                prop:value=move || {
                    state.get().0.get(&key1).map(|x| x.to_string()).unwrap_or_default()
                }
                on:input=move |e| {
                    set_state
                        .update(|s| {
                            s.0.insert(key.to_string(), event_target_value(&e));
                        })
                }
            >
                {options.iter().map(|v| view! { <option>{v.to_string()}</option> }).collect_view()}

            </select>
        </Flex>
    }
}

#[component]
pub fn ListItem(
    label: String,
    children: Children,
) -> impl IntoView {
    view! {
        <Flex>
            <Label>{label}</Label>
            {children()}
        </Flex>
    }
}
