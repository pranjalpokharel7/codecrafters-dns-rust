use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeserializationError {
    #[error("error deserializing message field: {0}")]
    MalformedField(&'static str),

    #[error("encountered unexpected EOF")]
    UnexpectedEOF,

    #[error("invalid compression pointer")]
    InvalidCompressionPointer,

    #[error("compression too deep: exiting before infinite recursion")]
    CompressionTooDeep,
}