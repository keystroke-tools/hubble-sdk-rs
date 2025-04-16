use crate::{allocator, error, host, types};

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
