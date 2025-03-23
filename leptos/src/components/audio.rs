//! Audio Streamer component

use leptos::prelude::*;
use leptos_use::{use_user_media_with_options, UseUserMediaOptions, UseUserMediaReturn};
use thaw::*;

use crate::css::styles;

#[component]
pub fn AudioStream() -> impl IntoView {
    let node = NodeRef::<leptos::html::Audio>::new();

    let start_rec = RwSignal::new(true);
    let result = RwSignal::new_local("".to_string());

    let options = UseUserMediaOptions::default().video(false).audio(true);

    let UseUserMediaReturn {
        stream,
        start,
        stop,
        ..
    } = use_user_media_with_options(options);

    start();

    let effect = Effect::watch(
        move || start_rec.get(),
        move |val, _prev, _| {
            if !val {
                tracing::info!("Stopping recording.");
                stop();
            } else {
                tracing::info!("Starting recording.");
                start();
            }
        },
        false,
    );

    Effect::new(move |_| {
        tracing::info!("State of the recording: {}.", start_rec.get_untracked());
        node.get().map(|v| match stream.get() {
            Some(Ok(s)) => v.set_src_object(Some(&s)),
            Some(Err(e)) => tracing::error!("Failed to get media stream: {e:?}"),
            None => tracing::debug!("No stream yet"),
        });
    });

    view! {
        <Space vertical=true>
            <audio node_ref=node controls  />
            <Checkbox checked=start_rec label="Start Record" />
            <div> { start_rec } </div>
        </Space>
    }
}
