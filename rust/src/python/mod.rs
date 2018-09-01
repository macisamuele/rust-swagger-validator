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
use pyo3::PyDict;
use pyo3::PyRawObject;
use swagger_schema::SwaggerSchema;

#[macro_use]
mod pyo3_built;
#[macro_use]
mod object_macros;

#[pyfunction]
pub fn convert_string(a: i64) -> String {
    format!("{}", a).to_string()
}

#[pyfunction]
pub fn no_parameters() -> usize {
    42
}

#[pyclass(subclass)]
struct SwaggerSpec {
    token: PyToken,
    swagger_schema: SwaggerSchema,
}

#[pymethods]
impl SwaggerSpec {
    #[new]
    fn __new__(_obj: &PyRawObject, _base_url: &str, _swagger_spec_dict: &PyDict) -> PyResult<()> {
        // TODO: find a way to convert PyDict into serde_json::Value
        unimplemented!();
    }

    #[classmethod]
    #[args(follow_references = false)]
    fn from_url(
        cls: &PyType,
        py: Python,
        url: &str,
        follow_references: bool,
    ) -> PyResult<PyObject> {
        if follow_references {
            unimplemented!();
        }

        initialize_python_object!(py, cls, |token| Self {
            token,
            swagger_schema: SwaggerSchema::new_from_url(url).unwrap()
        })
    }

    #[getter]
    fn uri(&self) -> PyResult<String> {
        match &self.swagger_schema.uri {
            Some(z) => Ok(z.to_owned().to_string()),
            None => panic!("None"),
        }
    }
}

#[pymodinit]
fn _rust_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__build__", pyo3_built!(py))?;
    m.add_class::<SwaggerSpec>()?;

    m.add_function(wrap_function!(convert_string))?;
    m.add_function(wrap_function!(no_parameters))?;

    Ok(())
}
