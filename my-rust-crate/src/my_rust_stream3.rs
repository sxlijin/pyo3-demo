use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::get_runtime as get_tokio_runtime;
use std::time::Duration;

#[pyclass]
pub struct MyRustStream3 {}

#[pymethods]
impl MyRustStream3 {
    #[staticmethod]
    fn new<'py>(py: Python<'py>, bridge_ref: &Bound<'py, PyAny>) -> PyResult<()> {
        let bridge_ref = bridge_ref.clone().unbind();

        get_tokio_runtime().spawn(async move {
            for i in 0..100 {
                Python::with_gil(|py| {
                    let callback = bridge_ref.clone_ref(py).into_bound(py);

                    // Call the callback function!
                    callback
                        .call1((format!("Sending an event from Rust: i={i}"),))
                        .unwrap();
                });
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });

        Ok(())
    }
}
