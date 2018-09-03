// Features required due to pyo3 -> https://github.com/PyO3/pyo3/issues/5 and https://github.com/PyO3/pyo3/issues/210
#![feature(extern_prelude, specialization)]
// Features required due to scoped clippy lints
#![feature(tool_lints)]
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

//// Uncomment the following lines to support debug of macros
//#![feature(trace_macros)]
//trace_macros!(true);

#[macro_use]
mod errors;
pub mod loaders;
pub mod swagger_schema;

#[cfg(feature = "python_bindings")]
#[macro_use]
extern crate pyo3;

#[cfg(feature = "python_bindings")]
pub mod python;
