mod my_ctrlc_handler;
mod my_ping_server;
mod my_rust_object;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn hello_world() -> PyResult<()> {
    println!("Hello, world!");
    Ok(())
}

#[pyfunction]
fn modify_string(text: &str) -> PyResult<String> {
    let text = format!("From Rust: {}", text);
    Ok(text)
}

#[pymodule]
fn my_rust_crate(m: Bound<'_, PyModule>) -> PyResult<()> {
    my_ctrlc_handler::install_ctrlc_handler();
    // panic!("Uh-oh - my Rust code has panicked!");
    // m.add_wrapped(wrap_pyfunction!(my_ping_server::start_server_via_tokio))?;
    m.add_class::<my_rust_object::MyRustObject>()?;
    Ok(())
}
