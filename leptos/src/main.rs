use tracing_subscriber::fmt;
use tracing_subscriber_wasm::MakeConsoleWriter;

mod app;
mod storage;
mod components;

fn main() {
    fmt()
        .with_writer(MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG))
        // For some reason, if we don't do this in the browser, we get a runtime error.
        .without_time()
        .init();

    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(app::App);
}
