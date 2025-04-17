use crate::{
    allocator,
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
        return Err(error::Error::MemoryAllocationFailed {
            context: "update_chunk".to_string(),
        });
    }

    // Write message to shared memory
    allocator::write_to_memory(ptr, &message);

    let _ = unsafe { host::update_entry(ptr, size) };

    Ok(())
}

/// Create chunks for an entry
///
/// Entry chunks are blocks/fragments of the document independently indexed and retrievable.
pub fn create_chunks(opts: types::CreateChunksOpts) -> Result<u64, error::Error> {
    let message = opts.to_capnp_message()?;

    let size = message.len() as u32;
    let ptr = allocator::allocate(size);
    if ptr == 0 {
        return Err(Error::MemoryAllocationFailed {
            context: "create_chunks".to_string(),
        });
    }
    allocator::write_to_memory(ptr, &message);

    let count = unsafe { host::create_entry_chunks(ptr, size) };
    Ok(count)
}
