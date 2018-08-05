#![feature(use_extern_macros)]

#[macro_use]
extern crate pyo3;

use pyo3::prelude::*;
//use pyo3::py::modinit;

#[macro_use]
mod pyo3_built;

fn convert_string(a: i64) -> String {
    format!("{}", a).to_string()
}

#[pymodinit]
fn _rust_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__build__", pyo3_built!(py))?;

    #[pyfn(m, "convert_string")]
    fn convert_string_py(a: i64) -> PyResult<String> {
        let out = convert_string(a);
        return Ok(out);
    }

    #[pyfn(m, "no_parameters")]
    fn no_parameters() -> PyResult<usize> {
        return Ok(42);
    }

    Ok(())
}