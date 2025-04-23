use crate::{
    allocator,
    error::{self, Error},
    host, safe_alloc, types,
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
    let ptr = safe_alloc!("update_chunk", size);

    // Write message to shared memory
    allocator::write_to_memory(ptr, &message);

    let _ = unsafe { host::entry_update(ptr, size) };

    Ok(())
}

/// Create chunks for an entry
///
/// Entry chunks are blocks/fragments of the document independently indexed and retrievable.
pub fn create_chunks(opts: types::CreateChunksOpts) -> Result<u64, error::Error> {
    let message = opts.to_capnp_message()?;

    let size = message.len() as u32;
    let ptr = safe_alloc!("create_chunks", size);
    allocator::write_to_memory(ptr, &message);

    let count = unsafe { host::entry_create_chunks(ptr, size) };
    Ok(count)
}
