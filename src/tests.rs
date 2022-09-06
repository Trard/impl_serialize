use crate as impl_serialize;
use serde::{ser, Serializer};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
enum SerializationError {
    #[error("Expected value higher {than}. Found {current_value}")]
    ExpectedValueHigher {
        than: i64,
        current_value: i64
    },
    #[error("Cannot serialize")]
    CannotSerialize,
    #[error("Cannot serialize value from char {0}")]
    CannotSerializeFromChar(char),
    #[error("Cannot serialize value from {0}")]
    CannotSerializeFrom(String),
    #[error("Custom({0})")]
    Custom(String),
}

impl serde::ser::Error for SerializationError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        SerializationError::Custom(msg.to_string())
    }
}

mod general;
mod metavariables;