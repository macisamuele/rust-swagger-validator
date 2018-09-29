#![deny(
    anonymous_parameters,
    bad_style,
    missing_copy_implementations,
    missing_debug_implementations,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    unused_qualifications
)]

extern crate rust_swagger_validator;

mod test_loaders;
#[cfg(feature = "python_bindings")]
mod test_python;
mod test_swagger_schema;
