use crate::error::Error;
use crate::{allocator, host};
use crate::{safe_alloc, types};

/// Sends a network request to the specified URL with the given method and body.
pub fn request(opts: types::RequestOpts) -> Result<types::NetworkResponse, Error> {
    let message = opts.to_capnp_message()?;

    let size = message.len() as u32;
    let ptr = safe_alloc!("write_request_data", size);

    // Write the message to memory
    allocator::write_to_memory(ptr, &message);

    let encoded_ptr = unsafe { host::network_request(ptr, size) };
    let (out_ptr, out_size) = allocator::decode_encoded_ptr("request", encoded_ptr)?;

    // Read the response from memory
    types::NetworkResponse::read_from_memory(out_ptr, out_size)
}
