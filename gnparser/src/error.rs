use thiserror::Error;

/// List of error types used in the library.
#[derive(Error, Debug)]
pub enum GNParserError {
    /// Indicates that a user entered a string that cannot be
    /// converted to a Method type. In such case the default format
    /// (Method::Restful) wil be used.
    #[error("cannot convert {method:?} to a method value")]
    InvalidMethodInput { method: String },
}

