use super::stream_impls::get_http_stream;
use futures::pin_mut;
use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::get_runtime as get_tokio_runtime;
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;

#[pyclass]
pub struct CallbackDrivenStream {
    pub join_handle: JoinHandle<()>,
}

#[pymethods]
impl CallbackDrivenStream {
    /// This strategy means that we don't have to create multiple Rust futures
    /// with references to the same `Stream` object!
    #[new]
    fn new<'py>(callback_ref: &Bound<'py, PyAny>, url: String) -> PyResult<Self> {
        let callback_ref = callback_ref.clone().unbind();

        let join_handle = get_tokio_runtime().spawn(async move {
            let s = get_http_stream(&url);
            pin_mut!(s); // async mumbo jumbo

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

        Ok(Self { join_handle })
    }
}
