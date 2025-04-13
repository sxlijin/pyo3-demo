use pyo3::prelude::*;

#[pyclass]
pub struct MyRustObject {
    text: String,
}

#[pymethods]
impl MyRustObject {
    #[staticmethod]
    fn my_static_method(text: String) -> Self {
        Self {
            text: format!("Created via static method: {}", text),
        }
    }

    #[new]
    fn my_constructor(text: String) -> Self {
        Self {
            text: format!("Created via constructor: {}", text),
        }
    }

    fn print(&self) -> PyResult<()> {
        println!("{}", self.text);
        Ok(())
    }
}
