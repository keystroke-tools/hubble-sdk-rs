use std::alloc::{Layout, alloc, dealloc};

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
pub unsafe fn ptr_to_string(ptr: u32, len: u32) -> String {
    let slice = unsafe { std::slice::from_raw_parts_mut(ptr as *mut u8, len as usize) };
    let utf8 = unsafe { std::str::from_utf8_unchecked_mut(slice) };
    utf8.to_owned()
}
/// Returns a pointer and size pair for the given string.
///
/// ## Safety
/// This function is unsafe because it returns a pointer to the string's
/// internal buffer, which may not be valid if the string is dropped or
pub unsafe fn string_to_ptr(s: &str) -> (u32, u32) {
    (s.as_ptr() as u32, s.len() as u32)
}

/// Reads a pointer and length from an encoded u64 value.
/// Format: the upper 32 bits represent the pointer, and the lower 32 bits represent the length
pub fn read_ptr_len(encoded: u64) -> (u32, u32) {
    let ptr = (encoded >> 32) as u32;
    let len = (encoded & 0xFFFFFFFF) as u32;
    (ptr, len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ptr_len_encoding() {
        let ptr = 0xABCD1234;
        let len = 1024;
        let packed = ((ptr as u64) << 32) | (len as u64);
        let (decoded_ptr, decoded_len) = read_ptr_len(packed);
        assert_eq!(ptr, decoded_ptr);
        assert_eq!(len, decoded_len);
    }
}
