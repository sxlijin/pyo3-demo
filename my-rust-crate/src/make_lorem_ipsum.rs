use pyo3::{exceptions::PyRuntimeError, prelude::*};
use serde::Deserialize;

#[pyfunction]
/// Make a Lorem Ipsum string by converting a pure Python object to a Rust object.
pub fn make_lorem_ipsum_from_python(options: &Bound<'_, PyAny>) -> PyResult<String> {
    let options = options.extract()?;
    make_lorem_ipsum(&options)
}

#[pyfunction]
/// Make a Lorem Ipsum string from a Python object implemented in Rust.
pub fn make_lorem_ipsum(options: &RustOptionsObject) -> PyResult<String> {
    let mut seed = "Lorem ipsum dolor sit amet.".to_string();
    if options.crab_emoji {
        seed.push_str("🦀");
    }
    if options.newlines {
        seed.push_str("\n");
    }

    Ok(seed.repeat(options.repeat))
}

#[derive(Deserialize, FromPyObject)]
#[pyclass]
pub struct RustOptionsObject {
    pub repeat: usize,
    pub crab_emoji: bool,
    pub newlines: bool,
}

#[pymethods]
impl RustOptionsObject {
    #[new]
    pub fn new(repeat: u32, crab_emoji: bool, newlines: bool) -> Self {
        Self {
            repeat: repeat as usize,
            crab_emoji,
            newlines,
        }
    }

    #[staticmethod]
    pub fn from_json_string(json_string: &str) -> PyResult<Self> {
        serde_json::from_str(json_string).map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}
