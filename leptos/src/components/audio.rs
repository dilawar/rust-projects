//! Audio Streamer component

use leptos::prelude::*;
use leptos_use::{use_user_media_with_options, UseUserMediaOptions, UseUserMediaReturn};
use thaw::*;
use web_sys::wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::MediaRecorder;

use crate::css::styles;

// Record audio
//
// Thanks you <https://github.com/wayeast/mediarecorder/blob/master/src/lib.rs> and chatgpt sucked
// lemons.
#[component]
pub fn AudioStream() -> impl IntoView {
    let node = NodeRef::<leptos::html::Audio>::new();

    let start_rec = RwSignal::new(true);
    let async_blob = RwSignal::new(None);

    // Create options to fetch only audio stream.
    let options = UseUserMediaOptions::default().video(false).audio(true);

    let UseUserMediaReturn {
        stream,
        start,
        stop,
        ..
    } = use_user_media_with_options(options);

    let on_data_available = Closure::wrap(Box::new(move |data: JsValue| {
        if let Ok(blob) = data.dyn_into::<web_sys::BlobEvent>() {
            if let Some(data) = blob.data() {
                // convert to gloo_file Blog.
                let blob = gloo_file::Blob::from(data);
                *async_blob.write() = Some(LocalResource::new(move || {
                    gloo_file::futures::read_as_bytes(&blob)
                }));
            }
        } else {
            tracing::info!(" bad data");
        }
    }) as Box<dyn FnMut(JsValue)>);

    Effect::new(move |_| {
        node.get().map(|v| match stream.get() {
            Some(Ok(s)) => {
                tracing::info!("Setting stream {s:?} to src...");
                v.set_src_object(Some(&s));
                let recorder = MediaRecorder::new_with_media_stream(&s).unwrap();
                recorder.set_ondataavailable(Some(on_data_available.as_ref().unchecked_ref()));
                recorder.start_with_time_slice(5000).unwrap();
            }
            Some(Err(e)) => tracing::error!("Failed to get media stream: {e:?}"),
            None => tracing::debug!("No stream yet"),
        });
    });

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
        false, /* Trigger it as soon as possible */
    );

    // Watch async_blob
    Effect::new(move |_| {
        if let Some(blob_resource) = async_blob.get() {
            tracing::info!("Got blob of buffer {:?}", blob_resource.read().as_deref());
        }
    });

    view! {
        <Space vertical=true>
            // Eventually I was to draw something related to audio stream here.
            <canvas />
            <audio node_ref=node controls />
            <Switch checked=start_rec label="Start Record" />
        </Space>
    }
}
