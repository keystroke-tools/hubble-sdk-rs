use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to read/parse Cap'n Proto message: {0}")]
    Capnp(capnp::Error),

    #[error("Failed to read UTF-8 string: {0}")]
    Utf8(std::str::Utf8Error),

    #[error("An I/O error occured: {0}")]
    Io(std::io::Error),

    #[error("A runtime error occured: {0}")]
    PluginError(String),

    #[error("Failed to read a string from memory: {reason}")]
    ReadStringError { reason: String },

    #[error("No valid UTF-8 string found in memory")]
    EmptyString,

    #[error("A memory-related error occured: {0}")]
    MemoryError(String),

    #[error("Failed to read memory in {context}")]
    ReadMemoryError { context: String },

    #[error("Failed to allocate memory in {context}")]
    MemoryAllocationFailed { context: String },

    #[error("Invalid arguments provided: {0}")]
    InvalidArguments(String),
}
