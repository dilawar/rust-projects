//! A sample forms

use strum::IntoEnumIterator;

use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_qr_scanner::Scan;
use leptos_use::storage::use_local_storage;
use reactive_stores::Store;
use thaw::*;

use crate::components::*;
use crate::css::styles;
use crate::storage::KeyVal;
use crate::storage::{GlobalState, GlobalStateStoreFields};

#[derive(Debug, strum::EnumIter, strum::Display)]
enum Gender {
    Male,
    Female,
    Other,
}

#[component]
pub fn Form() -> impl IntoView {
    let storage_key = RwSignal::new("".to_string());

    let (state, set_state, _) = use_local_storage::<KeyVal, JsonSerdeCodec>(storage_key);

    let upload_patient_consent_form = move |file_list: FileList| {
        let len = file_list.length();
        for i in 0..len {
            if let Some(file) = file_list.get(i) {
                tracing::info!("File to upload: {}", file.name());
            }
        }
    };

    view! {
        <h5>"Form"</h5>
        <Space vertical=true class=styles::ehr_list>

            // Everything starts with this key
            <ListItem label="Code".to_string()>
                <input bind:value=storage_key></input>
            </ListItem>

            // Patient
            <InputWithLabel key="phone".to_string() state set_state></InputWithLabel>
            <InputWithLabel key="name".to_string() state set_state></InputWithLabel>
            <SelectWithLabel
                key="gender".to_string()
                options=Gender::iter().map(|x| x.to_string()).collect()
                state
                set_state
            ></SelectWithLabel>
            <InputWithLabel key="extra".to_string() state set_state></InputWithLabel>

            <Upload custom_request=upload_patient_consent_form>
                <UploadDragger>"Drag file here"</UploadDragger>
            </Upload>

        </Space>
    }
}
