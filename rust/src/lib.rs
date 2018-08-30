//// Uncomment the following lines to support debug of macros
//#![feature(trace_macros)]
//trace_macros!(true);

#[macro_use]
mod errors;
pub mod loaders;
pub mod swagger_schema;

#[cfg(feature = "python_bindings")]
//#[macro_use]
extern crate pyo3;

#[cfg(feature = "python_bindings")]
pub mod python;
