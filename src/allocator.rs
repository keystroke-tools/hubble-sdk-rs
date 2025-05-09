use std::alloc::{Layout, alloc, dealloc};

use crate::error::Error;

/// Allocates size bytes and leaks the pointer where they start.
#[unsafe(no_mangle)]
pub extern "C" fn allocate(size: u32) -> u32 {
    if size == 0 {
        return 0;
    }

    let layout = Layout::from_size_align(size as usize, 8).expect("Failed to create layout");
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        panic!("Failed to allocate memory");
    }

    ptr as u32
}

// Deallocates the memory at the given pointer and size.
#[unsafe(no_mangle)]
pub extern "C" fn deallocate(ptr: u32, size: u32) {
    if ptr == 0 || size == 0 {
        return;
    }

    let layout = Layout::from_size_align(size as usize, 8).expect("Failed to create layout");
    unsafe {
        dealloc(ptr as *mut u8, layout);
    }
}

/// Returns a string from a pointer and length.
///
/// ## Safety
/// This function is unsafe because it dereferences a pointer and assumes
/// the memory is valid and contains a UTF-8 string.
///
/// NOTE: This function deallocates the memory at the given pointer after
/// reading the string.
pub unsafe fn ptr_to_string(ptr: u32, len: u32) -> String {
    let slice = unsafe { std::slice::from_raw_parts_mut(ptr as *mut u8, len as usize) };
    let str = unsafe { std::str::from_utf8_unchecked_mut(slice) }.to_owned();
    deallocate(ptr, len);
    str
}

/// Returns a buffer from a pointer and length.
///
/// ## Safety
/// This function is unsafe because it dereferences a pointer and assumes
pub unsafe fn ptr_to_buffer(ptr: u32, len: u32) -> Vec<u8> {
    let slice = unsafe { std::slice::from_raw_parts_mut(ptr as *mut u8, len as usize) };
    let buf = slice.to_vec();
    deallocate(ptr, len);
    buf
}

/// Returns a pointer and size pair for the given string.
///
/// ## Safety
/// This function is unsafe because it returns a pointer to the string's
/// internal buffer, which may not be valid if the string is dropped or
/// reallocated.
pub unsafe fn string_to_ptr(s: &str) -> (u32, u32) {
    (s.as_ptr() as u32, s.len() as u32)
}

/// Writes a buffer to memory at the given pointer.
pub fn write_to_memory(ptr: u32, data: &[u8]) {
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), ptr as *mut u8, data.len());
    }
}

/// Reads a pointer and length from an encoded u64 value.
/// Format: the upper 32 bits represent the pointer, and the lower 32 bits represent the length
pub fn decode_encoded_ptr(ctx: &str, encoded: u64) -> Result<(u32, u32), Error> {
    let ptr = (encoded >> 32) as u32;
    let len = (encoded & 0xFFFFFFFF) as u32;

    if ptr == 0 || len == 0 {
        return Err(Error::BadEncodedPointer {
            context: ctx.to_string(),
        });
    }

    Ok((ptr, len))
}

/// Encodes a pointer and size into a u64 value.
pub fn encode_ptr_with_size(ptr: u32, size: u32) -> u64 {
    ((ptr as u64) << 32) | (size as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ptr_len_encoding() {
        let ptr = 0xABCD1234;
        let len = 1024;
        let packed = ((ptr as u64) << 32) | (len as u64);
        if let Ok((decoded_ptr, decoded_len)) = decode_encoded_ptr("test", packed) {
            assert_eq!(ptr, decoded_ptr);
            assert_eq!(len, decoded_len);
        } else {
            panic!("Failed to decode pointer and length");
        }
    }
}
