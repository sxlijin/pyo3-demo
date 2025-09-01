use pyo3::{exceptions::PyRuntimeError, prelude::*};
use serde::Deserialize;

#[pyfunction]
pub fn make_lorem_ipsum(options: &LoremIpsumOptions) -> PyResult<String> {
    let mut seed = "Lorem ipsum dolor sit amet.".to_string();
    if options.crab_emoji {
        seed.push_str("🦀");
    }
    if options.newlines {
        seed.push_str("\n");
    }

    Ok(seed.repeat(options.repeat))
}

#[derive(Clone, Deserialize)]
#[pyclass]
pub struct LoremIpsumOptions {
    pub repeat: usize,
    pub crab_emoji: bool,
    pub newlines: bool,
}

#[pymethods]
impl LoremIpsumOptions {
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
