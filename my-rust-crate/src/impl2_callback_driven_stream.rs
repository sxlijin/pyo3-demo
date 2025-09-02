use super::stream_impls::get_http_stream;
use futures::pin_mut;
use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::get_runtime as get_tokio_runtime;
use tokio_stream::StreamExt;

/// This strategy means that we don't have to create multiple Rust futures
/// with references to the same `Stream` object!
#[pyfunction]
pub fn callback_driven_stream<'py>(url: String, callback_ref: &Bound<'py, PyAny>) -> PyResult<()> {
    let callback_ref = callback_ref.clone().unbind();

    get_tokio_runtime().spawn(async move {
        let s = get_http_stream(&url);
        pin_mut!(s); // pin the stream to the future's stack

        while let Some(event) = s.next().await {
            Python::with_gil(|py| {
                // pyo3 mumbo jumbo
                let py_callback = callback_ref.clone_ref(py).into_bound(py);

                // Call the callback function!
                py_callback
                    .call1((format!("Rust event: {event}"),))
                    .unwrap();
            });
        }
    });

    Ok(())
}
