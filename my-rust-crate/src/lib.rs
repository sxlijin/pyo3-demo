mod impl1_naive_stream;
mod impl2_callback_driven_stream;
mod make_lorem_ipsum;
mod stream_impls;

use pyo3::prelude::*;

#[pymodule]
fn my_rust_crate(m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<make_lorem_ipsum::RustOptionsObject>()?;
    m.add_function(wrap_pyfunction!(make_lorem_ipsum::make_lorem_ipsum, &m)?)?;
    m.add_function(wrap_pyfunction!(
        impl2_callback_driven_stream::callback_driven_stream,
        &m
    )?)?;

    Ok(())
}
