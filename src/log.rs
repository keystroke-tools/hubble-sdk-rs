use crate::{allocator, host};

/// Logs a debug message to the host environment.
pub fn debug(s: &str) {
    let (ptr, size) = unsafe { allocator::string_to_ptr(s) };
    unsafe { host::debug(ptr, size) };
}

/// Logs an error message to the host environment.
pub fn error(s: &str) {
    let (ptr, size) = unsafe { allocator::string_to_ptr(s) };
    unsafe { host::error(ptr, size) };
}

/// Logs a warning message to the host environment.
pub fn warn(s: &str) {
    let (ptr, size) = unsafe { allocator::string_to_ptr(s) };
    unsafe { host::warn(ptr, size) };
}
