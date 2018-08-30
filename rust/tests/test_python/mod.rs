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

use rust_swagger_validator::python::*;

#[test]
fn test_convert_string() {
    assert_eq!(convert_string(1), "1");
}

#[test]
fn test_return_42() {
    assert_eq!(return_42(), 42);
}
