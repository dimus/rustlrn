use super::GNParserError;
use strum_macros::Display;

/// Indicates method to parse data;
#[derive(Debug, Display, Clone)]
pub enum Method {
    /// Uses RESTful API
    Restful,
    /// Uses gRPC service
    GRPC,
    /// Uses gnparser as a shared C library
    Binary,
}

impl Default for Method {
    fn default() -> Self {
        Method::Restful
    }
}
impl Method {
    /// Creates a new format entity out of a string.
    pub fn new(m: &str) -> Result<Self, GNParserError> {
        match m {
            "restful" => Ok(Method::Restful),
            "grpc" => Ok(Method::GRPC),
            "binary" => Ok(Method::Binary),
            _ => Err(GNParserError::InvalidMethodInput {
                method: m.to_owned(),
            }),
        }
    }
}

#[test]
fn method_as_str() {
    assert_eq!(Method::Restful.to_string(), "Restful")
}

