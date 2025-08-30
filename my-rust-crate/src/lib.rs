mod april_18_srug;
mod impl1_naive_stream;
mod impl2_callback_driven_stream;
mod my_rust_object;
// mod my_rust_stream4;

use std::time::Duration;

use eventsource_stream::Eventsource;
use futures::future::Either;
use futures::stream::{self, Stream};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use tokio_stream::StreamExt as TokioStreamExt;

pub fn stream_values(url: &str) -> impl Stream<Item = String> {
    let url = url.to_string();

    let s = stream::once(async move {
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("Accept", "text/event-stream")
            .send()
            .await;

        match response {
            Ok(resp) => {
                // First emit success status
                let status_stream = stream::once(async { "Stream initialized".to_string() });

                // Then create the SSE event stream
                let sse_stream =
                    resp.bytes_stream()
                        .eventsource()
                        .filter_map(|result| match result {
                            Ok(event) => Some(event.data),
                            Err(_) => None,
                        });

                // Chain them together: status first, then SSE events
                Either::Left(status_stream.chain(sse_stream))
            }
            Err(e) => {
                // On error, emit error status and nothing else
                Either::Right(stream::once(async move {
                    format!("Failed to initialize stream: {}", e)
                }))
            }
        }
    });

    futures::StreamExt::flatten(s)
}

pub fn stream_values2(_url: &'static str) -> impl Stream<Item = String> {
    stream::repeat_with(|| format!("Hello world! {}", chrono::Local::now()))
        .throttle(Duration::from_secs(1))
}

#[pymodule]
fn my_rust_crate(m: Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_wrapped(wrap_pyfunction!(hello_world));
    // m.add_wrapped(wrap_pyfunction!(my_ping_server::start_server_via_asyncio))?;
    // m.add_wrapped(wrap_pyfunction!(sleep_blocking));
    custom_ctrlc_handler::install_ctrlc_handler();
    // panic!("Uh-oh - my Rust code has panicked!");
    m.add_wrapped(wrap_pyfunction!(my_ping_server::start_server_via_tokio))?;
    m.add_class::<my_rust_object::MyRustObject>()?;
    // m.add_class::<my_rust_stream::MyRustStream>()?;
    m.add_class::<impl2_callback_driven_stream::CallbackDrivenStream>()?;
    m.add_wrapped(wrap_pyfunction!(blocking_fetch_sse_stream))?;
    Ok(())
}
