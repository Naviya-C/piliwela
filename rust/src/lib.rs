use pyo3::prelude::*;

#[pyfunction]
fn hello() -> String {
    "Ayubowan! from Piliwela".to_string()
}

#[pymodule]
fn piliwela(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}