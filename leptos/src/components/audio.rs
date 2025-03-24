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

    // Create options to fetch only audio stream.
    let options = UseUserMediaOptions::default().video(false).audio(true);

    let UseUserMediaReturn {
        stream,
        start,
        stop,
        ..
    } = use_user_media_with_options(options);

    // start/stop recording 
    let _effect = Effect::watch(
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
        true, /* Trigger it as soon as possible */
    );

    Effect::new(move |_| {
        tracing::debug!("State of the recording: {}.", start_rec.get_untracked());
        node.get().map(|v| match stream.get() {
            Some(Ok(s)) => {
                tracing::debug!("Setting stream {s:?} to src...");
                v.set_src_object(Some(&s));
            },
            Some(Err(e)) => tracing::error!("Failed to get media stream: {e:?}"),
            None => tracing::debug!("No stream yet"),
        });
    });

    view! {
        <Space vertical=true>
            // Eventually I was to draw something related to audio stream here.
            <canvas />
            <audio node_ref=node controls  />
            <Switch checked=start_rec label="Start Record" />
        </Space>
    }
}
