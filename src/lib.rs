pub mod allocator;
pub mod error;
pub(crate) mod host;
pub mod log;
pub mod macros;
pub mod types;

use crate::error::Error;

pub mod entry_capnp {
    include!(concat!(env!("OUT_DIR"), "/entry_capnp.rs"));
}

pub mod store_capnp {
    include!(concat!(env!("OUT_DIR"), "/store_capnp.rs"));
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

// pub fn on_create(ptr: u32, len: u32) {
//     let entry = capnp_message_to_type!(ptr, len, entry_capnp::entry::Reader, types::entry::Entry);
// }
