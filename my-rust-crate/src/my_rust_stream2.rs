use pyo3::exceptions::PyStopAsyncIteration;
use pyo3::prelude::*;
use std::time::Duration;

#[pyclass]
pub struct MyRustStream2 {
    rx: tokio::sync::mpsc::UnboundedReceiver<String>,
}

#[pymethods]
impl MyRustStream2 {
    #[new]
    fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        tokio::spawn(async move {
            for i in 0..100 {
                tx.send(format!("Event {i}")).expect("Error sending event");
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });

        Self { rx }
    }

    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let event = self.rx.recv().await;

            match event {
                Some(s) => Ok(s),
                None => Err(PyStopAsyncIteration::new_err("Stream ended")),
            }
        })
    }
}
