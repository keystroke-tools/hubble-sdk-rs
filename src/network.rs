use crate::error::Error;
use crate::types;
use crate::{allocator, host};

/// Sends a network request to the specified URL with the given method and body.
pub fn request(opts: types::RequestOpts) -> Result<types::NetworkResponse, Error> {
    let message = opts.to_capnp_message()?;

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
    types::NetworkResponse::read_from_memory(out_ptr, out_size)
}
