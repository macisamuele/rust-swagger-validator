#[macro_use]
extern crate pyo3;

use pyo3::prelude::*;

#[macro_use]
mod errors;
mod pyo3_built;

fn convert_string(a: i64) -> String {
    format!("{}", a).to_string()
}

fn return_42() -> usize {
    42
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
        return Ok(return_42());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use convert_string;
    use return_42;

    #[test]
    fn test_convert_string() {
        assert_eq!(convert_string(1), "1");
    }

    #[test]
    fn test_return_42() {
        assert_eq!(return_42(), 42);
    }
}
