use crate::stream_values;
use futures::pin_mut;
use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::get_runtime as get_tokio_runtime;
use tokio_stream::StreamExt;

#[pyclass]
pub struct CallbackDrivenStream {}

#[pymethods]
impl CallbackDrivenStream {
    /// This strategy means that we don't have to create multiple Rust futures
    /// with references to the same `Stream` object!
    #[staticmethod]
    fn new<'py>(callback_ref: &Bound<'py, PyAny>) -> PyResult<()> {
        let callback_ref = callback_ref.clone().unbind();

        get_tokio_runtime().spawn(async move {
            let s = stream_values("https://sse.dev/test");
            pin_mut!(s);

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
}
