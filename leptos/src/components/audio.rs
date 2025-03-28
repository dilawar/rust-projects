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
    let canvas_node = NodeRef::<leptos::html::Canvas>::new();

    let start_rec = RwSignal::new(false);
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
        if let Some(v) = node.get() {
            match stream.get() {
                Some(Ok(s)) => {
                    tracing::info!("Setting stream {s:?} to src...");
                    v.set_src_object(Some(&s));
                    let recorder = MediaRecorder::new_with_media_stream(&s).unwrap();
                    recorder.set_ondataavailable(Some(on_data_available.as_ref().unchecked_ref()));
                    recorder.start_with_time_slice(500).unwrap();
                }
                Some(Err(e)) => tracing::error!("Failed to get media stream: {e:?}"),
                None => tracing::debug!("No stream yet"),
            }
        }
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
        true, /* Trigger it as soon as possible */
    );

    // Watch async_blob
    Effect::new(move |_| {
        let ctx = canvas_node
            .get()
            .unwrap()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        if let Some(blob_resource) = async_blob.get() {
            if let Some(Ok(data)) = blob_resource.read().as_deref() {
                tracing::info!("Got blob of buffer {data:?}");
                ctx.reset();
                ctx.begin_path();
                for (i, v) in data.iter().enumerate() {
                    ctx.line_to(
                        i as f64,
                        *v as f64 / 5.0 + 50.0, /* 50 is half of height of canvas */
                    )
                }
                ctx.stroke(); // render
            }
        }
    });

    view! {
        <Space vertical=true>
            // Eventually I was to draw something related to audio stream here.
            <canvas node_ref=canvas_node class=styles::canvas />
            <audio node_ref=node controls />
            <Switch checked=start_rec label="Start Record" />
            <div>"Record and plot every half second of data"</div>
        </Space>
    }
}
