#![cfg_attr(
    feature = "cargo-clippy",
// Allow lints that will fail due to PyO3
    allow(clippy::cast_ptr_alignment, clippy::transmute_ptr_to_ptr)
)]

use pyo3::exceptions::NotImplementedError;
use pyo3::exceptions::ValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::PyType;
use pyo3::IntoPyPointer;
use pyo3::PyRawObject;
use swagger_schema::SwaggerSchema as RustSwaggerSchema;
use swagger_schema::SwaggerSchemaError as RustSwaggerSchemaError;

#[macro_use]
mod pyo3_built;
#[macro_use]
mod object_macros;

#[pyfunction]
pub fn convert_string(a: i64) -> String {
    format!("{}", a)
}

#[pyfunction]
pub fn no_parameters() -> usize {
    42
}

#[pyclass(subclass)]
struct RustBase {
    token: PyToken,
    #[prop(get)]
    val1: u32,
}

#[pymethods]
impl RustBase {
    #[new]
    fn __new__(obj: &PyRawObject) -> PyResult<()> {
        obj.init(|token| Self { val1: 10, token })
    }

    pub fn method(&self) -> PyResult<()> {
        Ok(())
    }
}

#[pyclass(subclass, extends=RustBase)]
struct RustSwaggerSpec {
    token: PyToken,
    swagger_schema: RustSwaggerSchema,
}

#[pymethods]
impl RustSwaggerSpec {
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
            let err = PyErr::new::<NotImplementedError, _>(
                "follow_references is not implemented yet".to_string(),
            );
            return Err(err);
        }

        let swagger_schema = RustSwaggerSchema::new_from_url(url)?;

        initialize_python_object!(py, cls, |token| Self {
            token,
            swagger_schema
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

impl From<RustSwaggerSchemaError> for PyErr {
    fn from(err: RustSwaggerSchemaError) -> Self {
        // TODO: make this more descriptive and extracting a credible exception
        Self::new::<ValueError, _>(format!("{:?}", err))
    }
}

#[pymodinit]
fn _rust_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__build__", pyo3_built!(py))?;
    m.add_class::<RustBase>()?;
    m.add_class::<RustSwaggerSpec>()?;

    m.add_function(wrap_function!(convert_string))?;
    m.add_function(wrap_function!(no_parameters))?;

    Ok(())
}
