#![deny(
    anonymous_parameters,
    bad_style,
    missing_copy_implementations,
    missing_debug_implementations,
// missing_docs, // TODO: add missing_docs to list of deny
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    unused_qualifications,
)]

#[macro_export]
macro_rules! from_error_to_enum_variant {
    // Define association between errors and equivalent enum errors representations
    // This will allow to reduce the amount of `match`es and use a simpler `?`
    // Example of usage:
    //      from_error_to_enum_variant!(io::Error, LoaderError, IOError);
    // To produce something like
    //      impl From<$io::Error> for LoaderError {
    //          fn from(error: io::Error) -> Self {
    //              LoaderError::IOError(error)
    //          }
    //      }
    ($original_error:ty, $enum_error_class:tt, $enum_variant:ident) => {
        impl From<$original_error> for $enum_error_class {
            fn from(error: $original_error) -> Self {
                $enum_error_class::$enum_variant(error)
            }
        }
    };
}
