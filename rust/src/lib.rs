// Features required due to pyo3 -> https://github.com/PyO3/pyo3/issues/5 and https://github.com/PyO3/pyo3/issues/210
#![feature(specialization)]
// Features required due to scoped clippy lints
#![deny(
    // unreachable_pub,
    anonymous_parameters,
    bad_style,
    const_err,
    dead_code,
    deprecated,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    incoherent_fundamental_impls,
    late_bound_lifetime_arguments,
    missing_copy_implementations,
    missing_debug_implementations,
    // missing_docs,
    non_shorthand_field_patterns,
    non_upper_case_globals,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unreachable_code,
    unreachable_patterns,
    unsafe_code,
    unused_allocation,
    unused_assignments,
    unused_comparisons,
    unused_doc_comments,
    unused_extern_crates,
    unused_extern_crates,
    unused_import_braces,
    unused_import_braces,
    unused_imports,
    unused_macros,
    unused_parens,
    unused_qualifications,
    unused_results,
    unused_unsafe,
    unused_variables,
    warnings,
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
