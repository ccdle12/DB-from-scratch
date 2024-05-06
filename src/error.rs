use std::{fmt, io, string::FromUtf8Error};

/// Error is the main error type for this library.
#[derive(Debug)]
pub enum Error {
    SerdeJson(serde_json::Error),
    IOError(io::Error),
    UTF8Error(FromUtf8Error),
}

/// Helper macro that implements Display for each Error variant.
macro_rules! impl_display {
    ($($error_variant:path),*) => {
        impl fmt::Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match *self {
                    $($error_variant(ref message) => write!(f, "{}", message),)*
                }
            }
        }
    };
}

impl_display!(Error::SerdeJson, Error::IOError, Error::UTF8Error);

/// Helper macro that implements From for each external Error to crate Error variant.
macro_rules! impl_error_conversions {
    ($($error_type:path => $error_variant:path),*) => {
        $(impl From<$error_type> for Error {
            fn from(err: $error_type) -> Error {
                $error_variant(err)
            }
        })*
    };
}

impl_error_conversions!(
    serde_json::Error => Error::SerdeJson,
    io::Error => Error::IOError,
    FromUtf8Error => Error::UTF8Error
);

pub type Result<T> = std::result::Result<T, Error>;
