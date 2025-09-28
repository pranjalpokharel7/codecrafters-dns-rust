use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeserializationError {
    #[error("error deserializing message field: {0}")]
    MalformedField(&'static str),

    #[error("encountered unexpected EOF")]
    UnexpectedEOF,
}