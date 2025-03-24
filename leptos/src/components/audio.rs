//! Audio Streamer component

use leptos::prelude::*;
use leptos_use::{
    use_interval, use_user_media_with_options, UseIntervalReturn,
    UseUserMediaOptions, UseUserMediaReturn,
};
use thaw::*;

use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;

use crate::css::styles;

#[component]
pub fn AudioStream() -> impl IntoView {
    let node = NodeRef::<leptos::html::Audio>::new();

    let start_rec = RwSignal::new(true);
    let pcm_data = RwSignal::new(vec![]);

    Effect::new(move |_| {
        // Get the default audio host and input device.
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .expect("No input device available");
        let config = device
            .default_input_config()
            .expect("Failed to get input config");

        // start the audio stream
        let stream: cpal::Stream = device
            .build_input_stream(
                &config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    pcm_data.update(|samples| {
                        samples.clear();
                        samples.extend_from_slice(data);
                    });
                },
                |err| tracing::error!("Stream error: {err}"),
                None,
            )
            .expect("Failed to build input stream");

        stream.play().expect("failed to play stream");
    });

    view! {
        // Eventually I was to draw something related to audio stream here.
        <Switch checked=start_rec label="Start Record" />
    }
}
