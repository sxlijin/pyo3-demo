mod my_ctrlc_handler;
mod my_ping_server;
mod my_rust_object;
// mod my_rust_stream;
// mod my_rust_stream2;
mod my_rust_stream3;

use std::time::Duration;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use tokio::time::sleep;

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

#[pyfunction]
fn sleep_blocking() {
    let t = tokio::runtime::Runtime::new().unwrap();
    t.block_on(tokio::time::sleep(Duration::from_secs(5)));
}

#[pymodule]
fn my_rust_crate(m: Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_wrapped(wrap_pyfunction!(hello_world));
    // m.add_wrapped(wrap_pyfunction!(my_ping_server::start_server_via_asyncio))?;
    // m.add_wrapped(wrap_pyfunction!(sleep_blocking));
    my_ctrlc_handler::install_ctrlc_handler();
    // panic!("Uh-oh - my Rust code has panicked!");
    m.add_wrapped(wrap_pyfunction!(my_ping_server::start_server_via_tokio))?;
    m.add_class::<my_rust_object::MyRustObject>()?;
    // m.add_class::<my_rust_stream::MyRustStream>()?;
    m.add_class::<my_rust_stream3::MyRustStream3>()?;
    Ok(())
}
