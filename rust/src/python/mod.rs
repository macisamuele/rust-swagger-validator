#![deny(
    anonymous_parameters,
    bad_style,
    missing_copy_implementations,
    missing_debug_implementations,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    unused_qualifications,
)]

use pyo3::prelude::*;

#[macro_use]
mod pyo3_built;

pub fn convert_string(a: i64) -> String {
    format!("{}", a).to_string()
}

pub fn return_42() -> usize {
    42
}

#[pymodinit]
fn _rust_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__build__", pyo3_built!(py))?;

    #[pyfn(m, "convert_string")]
    fn convert_string_py(a: i64) -> PyResult<String> {
        let out = convert_string(a);
        Ok(out)
    }

    #[pyfn(m, "no_parameters")]
    fn no_parameters() -> PyResult<usize> {
        Ok(return_42())
    }

    Ok(())
}
