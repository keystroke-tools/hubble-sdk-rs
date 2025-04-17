use crate::{
    allocator, entry_capnp,
    error::{self, Error},
    host, types,
};

/// Update an entry in the data store.
pub fn update(opts: types::UpdateEntryOpts) -> Result<(), error::Error> {
    if opts.name.is_none() && opts.content.is_none() && opts.checksum.is_none() {
        return Err(error::Error::InvalidArguments(
            "At least one of name, content, or checksum must be provided".to_string(),
        ));
    }

    let message = opts.to_capnp_message()?;
    let size = message.len() as u32;
    let ptr = allocator::allocate(size);
    if ptr == 0 {
        return Err(error::Error::MemoryAllocationFailed);
    }

    // Write message to shared memory
    allocator::write_to_memory(ptr, &message);

    let _ = unsafe { host::update_entry(ptr, size) };

    Ok(())
}

/// Create chunks for an entry
///
/// Entry chunks are blocks/fragments of the document independently indexed and retrievable.
pub fn create_chunks(opts: types::CreateChunksOpts) -> Result<i64, error::Error> {
    let message = opts.to_capnp_message()?;

    let size = message.len() as u32;
    let ptr = allocator::allocate(size);
    if ptr == 0 {
        return Err(Error::MemoryAllocationFailed);
    }
    allocator::write_to_memory(ptr, &message);

    let encoded_ptr = unsafe { host::create_entry_chunks(ptr, size) };
    let (out_ptr, out_size) = allocator::read_ptr_len(encoded_ptr);
    if out_ptr == 0 || out_size == 0 {
        return Err(Error::MemoryAllocationFailed);
    }

    let result = unsafe { allocator::ptr_to_string(out_ptr, out_size) };
    if result.is_empty() {
        return Err(Error::EmptyString);
    }

    result.parse::<i64>().map_err(|_| {
        error::Error::InvalidArguments("Failed to parse chunk count from response".to_string())
    })
}
