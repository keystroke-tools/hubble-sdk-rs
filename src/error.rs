use thiserror::Error;

use crate::allocator;

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

    #[error("Failed to decode pointer in {context}")]
    BadEncodedPointer { context: String },

    #[error("Invalid arguments provided: {0}")]
    InvalidArguments(String),

    #[error("{0}")]
    StoreError(StoreError),

    #[error("Invalid rand value size: {expected} bytes, got {actual} bytes")]
    BadRandomSize { expected: u32, actual: u32 },
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("Key not found in store: {key}")]
    KeyNotFound { key: String },

    #[error("Host returned an unexpected result")]
    UnexpectedResult,

    #[error("Failed to delete key: {key}, reason: {reason}")]
    FailedDelete { key: String, reason: String },
}

impl Error {
    /// Writes an error string to a shared memory space and returns an encoded pointer.
    pub fn write_to_host(&self) -> u64 {
        let (ptr, size) = unsafe { allocator::string_to_ptr(self.to_string().as_str()) };
        allocator::encode_ptr_with_size(ptr, size)
    }
}
