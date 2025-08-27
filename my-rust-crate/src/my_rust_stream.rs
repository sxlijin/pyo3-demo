use futures::stream;
use pyo3::exceptions::PyStopAsyncIteration;
use pyo3::prelude::*;
use std::pin::Pin;
use std::time::Duration;
use tokio_stream::StreamExt;

#[pyclass]
pub struct MyRustStream {
    stream: Pin<Box<dyn futures::Stream<Item = String> + Send + Sync>>,
}

#[pymethods]
impl MyRustStream {
    #[new]
    fn new() -> Self {
        Self {
            stream: Box::pin(
                stream::repeat_with(|| format!("Hello world! {}", chrono::Local::now()))
                    .throttle(Duration::from_secs(1)),
            ),
        }
    }

    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let event = self.stream.next().await;

            match event {
                Some(s) => Ok(s),
                None => Err(PyStopAsyncIteration::new_err("Stream ended")),
            }
        })
    }
}
