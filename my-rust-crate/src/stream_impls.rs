use std::time::Duration;

use eventsource_stream::Eventsource;
use futures::future::Either;
use futures::stream::{self, Stream};
use tokio_stream::StreamExt as TokioStreamExt;

pub fn get_http_stream(url: &str) -> impl Stream<Item = String> {
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

pub fn hello_world_stream() -> impl Stream<Item = String> {
    stream::repeat_with(|| format!("Hello world! {}", chrono::Local::now()))
        .throttle(Duration::from_secs(1))
}
