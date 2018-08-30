//// Uncomment the following lines to support debug of macros
//#![feature(trace_macros)]
//trace_macros!(true);

#[macro_use]
mod errors;
pub mod loaders;
pub mod swagger_schema;

//#[macro_use]
extern crate pyo3;

pub mod python;
