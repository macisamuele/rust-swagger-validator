#[macro_export]
macro_rules! from_error_to_enum_variant {
    // Define association between errors and equivalent enum errors representations
    // This will allow to reduce the amount of `match`es and use a simpler `?`
    //
    // Example of usage 1:
    //      from_error_to_enum_variant!(io::Error, LoaderError, IOError);
    // is equivalent to write
    //      impl From<$io::Error> for LoaderError {
    //          fn from(error: io::Error) -> Self {
    //              LoaderError::IOError(error)
    //          }
    //      }
    // Example of usage 2:
    //      from_error_to_enum_variant!(url::ParseError, LoaderError, InvalidURL, |error| UrlError::ParseError(error));
    // is equivalent to write
    //      impl From<url::ParseError> for LoaderError {
    //          fn from(error: url::ParseError) -> Self {
    //              LoaderError::InvalidURL(
    //                  (|error| UrlError::ParseError(error))(error)
    //              )
    //          }
    //      }
    //
    ($original_error:ty, $enum_error_class:tt, $enum_variant:ident) => {
        impl From<$original_error> for $enum_error_class {
            fn from(error: $original_error) -> Self {
                $enum_error_class::$enum_variant(error)
            }
        }
    };

    ($original_error:ty, $enum_error_class:tt, $enum_variant:ident, $error_builder:expr) => {
        impl From<$original_error> for $enum_error_class {
            fn from(error: $original_error) -> Self {
                $enum_error_class::$enum_variant($error_builder(error))
            }
        }
    };
}

macro_rules! enum_with_automatic_from_trait_implementation {
    // Define association between errors and equivalent enum errors representations
    // This will allow to reduce the amount of `match`es and use a simpler `?`
    //
    // Example of usage 1:
    //      enum_with_automatic_from_trait_implementation!(EnumName, A(i32), B(bool));
    // is equivalent to write
    // #[derive(Debug)]
    // pub enum EnumName {
    //     A(i32),
    //     B(bool)
    // }
    // impl From<i32> for EnumName {
    //     fn from(error: i32) -> Self {
    //         EnumName::A(error)
    //     }
    // }
    // impl From<bool> for EnumName {
    //     fn from(error: bool) -> Self {
    //         EnumName::B(error)
    //     }
    // }
    (
        $derive:meta,
        $enum_name:ident,
        $($variant_name:ident($inner_type:path)),+
    ) => (
            #[$derive]
            pub enum $enum_name {
                $(
                    $variant_name($inner_type)
                ),+
            }
            $(
                from_error_to_enum_variant!($inner_type, $enum_name, $variant_name);
            )+
    );
}
