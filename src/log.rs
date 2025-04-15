use crate::{allocator, host};

pub fn debug(s: &str) {
    let (ptr, size) = unsafe { allocator::string_to_ptr(s) };
    unsafe { host::debug(ptr, size) };
}

pub fn error(s: &str) {
    let (ptr, size) = unsafe { allocator::string_to_ptr(s) };
    unsafe { host::error(ptr, size) };
}
