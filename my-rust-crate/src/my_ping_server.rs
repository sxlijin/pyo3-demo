use axum::{
    extract::{self},
    response::{
        sse::{Event, Sse},
        IntoResponse, Response,
    },
    routing::get,
    Router,
};
use futures::stream;
use pyo3::{exceptions::PyRuntimeError, prelude::*};
use std::{convert::Infallible, time::Duration};
use tokio::net::TcpListener;
use tokio_stream::StreamExt;

#[derive(serde::Deserialize)]
pub struct PingQuery {
    stream: Option<bool>,
}

pub async fn ping_handler(extract::Query(query): extract::Query<PingQuery>) -> Response {
    let response = format!("pong (from {})", env!("CARGO_PKG_NAME"));
    match query.stream {
        Some(true) => {
            // Create an endless stream of "pong" messages
            let stream = stream::iter(0..)
                .map(move |i| {
                    Ok::<_, Infallible>(Event::default().data(format!("{}: seq {}", response, i)))
                })
                .throttle(Duration::from_millis(500));

            Sse::new(stream).into_response()
        }
        _ => format!("{}\n", response).into_response(),
    }
}

pub async fn start_server_impl(addr: &str) -> PyResult<()> {
    let app = Router::new().route("/", get(ping_handler));

    let tcp_listener = TcpListener::bind(addr)
        .await
        .map_err(|e| PyRuntimeError::new_err(format!("Error while binding to address: {}", e)))?;
    eprintln!("Rust HTTP server: listening on {}", addr);
    axum::serve(tcp_listener, app.into_make_service())
        .await
        .map_err(|e| PyRuntimeError::new_err(format!("Error while serving: {}", e)))?;

    Ok(())
}

#[pyfunction]
pub fn start_server_via_tokio(addr: String) -> PyResult<()> {
    let tokio_rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    tokio_rt.block_on(start_server_impl(&addr))
}

#[pyfunction]
pub fn start_server_via_asyncio(py: Python, addr: String) -> PyResult<Bound<PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move { start_server_impl(&addr).await })
}
