use pyo3::prelude::*;

#[pyclass]
pub struct MyRustObject {
    text: String,
    t: tokio::runtime::Runtime,
}

#[pymethods]
impl MyRustObject {
    #[new]
    fn new(text: String) -> Self {
        Self {
            text: format!("Created via constructor: {}", text),
            t: tokio::runtime::Runtime::new().unwrap(),
        }
    }

    #[staticmethod]
    fn my_static_method(text: String) -> Self {
        Self {
            text: format!("Created via static method: {}", text),
            t: tokio::runtime::Runtime::new().unwrap(),
        }
    }

    fn print(&self) -> PyResult<()> {
        println!("{}", self.text);
        Ok(())
    }
}
