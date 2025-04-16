pub mod allocator;
pub mod error;
pub(crate) mod host;
pub mod log;
pub mod macros;
pub mod types;

use types::NetworkResponse;

use crate::error::Error;

pub mod entry_capnp {
    include!(concat!(env!("OUT_DIR"), "/entry_capnp.rs"));
}

pub mod store_capnp {
    include!(concat!(env!("OUT_DIR"), "/store_capnp.rs"));
}

pub mod network_capnp {
    include!(concat!(env!("OUT_DIR"), "/network_capnp.rs"));
}

/// Chunks the input string into smaller pieces with overlap (to retain context).
pub fn chunk_with_overlap(s: &str) -> Result<Vec<String>, Error> {
    let (ptr, size) = unsafe { allocator::string_to_ptr(s) };
    let chunks = unsafe { host::chunk_with_overlap(ptr, size) };

    let (out_ptr, out_size) = allocator::read_ptr_len(chunks);

    read_chunk_result!(out_ptr, out_size)
}

/// Chunks the input string into smaller pieces by sentence.
pub fn chunk_by_sentence(s: &str) -> Result<Vec<String>, Error> {
    let (ptr, size) = unsafe { allocator::string_to_ptr(s) };
    let chunks = unsafe { host::chunk_by_sentence(ptr, size) };

    let (out_ptr, out_size) = allocator::read_ptr_len(chunks);

    read_chunk_result!(out_ptr, out_size)
}

/// Sends a network request to the specified URL with the given method and body.
pub fn request(opts: types::RequestOpts) -> Result<types::NetworkResponse, Error> {
    let message = opts.to_capnp().map_err(Error::Capnp)?;

    let len = message.len() as u32;
    let ptr = allocator::allocate(len);

    if ptr == 0 {
        return Err(Error::MemoryAllocationFailed);
    }

    // Write the message to memory
    allocator::write_to_memory(ptr, &message);

    let encoded_ptr = unsafe { host::network_request(ptr, len) };
    let (out_ptr, out_size) = allocator::read_ptr_len(encoded_ptr);
    if out_ptr == 0 || out_size == 0 {
        return Err(Error::MemoryAllocationFailed);
    }

    // Read the response from memory
    NetworkResponse::read_from_memory(out_ptr, out_size)
}
