use crate::{
    allocator,
    error::{Error, StoreError},
    host, store_capnp, types,
};

pub(crate) const NOT_FOUND_VALUE: &str = "__NOT_FOUND_0x0000__";

pub(crate) const DELETE_OK: &str = "OK";
pub(crate) const DELETE_ERR_PREFIX: &str = "ERR("; // Errors in the delete operation are returned
// as ERR(<error message>)

/// Gets a value from the store by its key.
pub fn get(key: &str) -> Result<String, Error> {
    let (ptr, size) = unsafe { allocator::string_to_ptr(key) };
    let encoded_ptr = unsafe { host::store_get(ptr, size) };
    let (out_ptr, out_size) = allocator::decode_encoded_ptr(encoded_ptr);
    if out_ptr == 0 || out_size == 0 {
        return Err(Error::StoreError(StoreError::UnexpectedResult));
    }

    let output = unsafe { allocator::ptr_to_string(out_ptr, out_size) };
    if output == NOT_FOUND_VALUE {
        return Err(Error::StoreError(StoreError::KeyNotFound {
            key: key.to_string(),
        }));
    }

    Ok(output)
}

/// Sets a value in the store for the current plugin.
///
/// If the key already exists, it will be overwritten, and the function will return the current
/// value.
pub fn set(key: &str, value: &str) -> Result<String, Error> {
    let message = types::StoreSetOpts::new(key, value).to_capnp_message()?;

    let size = message.len() as u32;
    let ptr = allocator::allocate(size);
    if ptr == 0 {
        return Err(Error::MemoryAllocationFailed {
            context: "write_request_data".to_string(),
        });
    }

    let encoded_ptr = unsafe { host::store_set(ptr, size) };
    let (out_ptr, out_size) = allocator::decode_encoded_ptr(encoded_ptr);
    if out_ptr == 0 || out_size == 0 {
        return Err(Error::MemoryAllocationFailed {
            context: "store_set".to_string(),
        });
    }

    let output = unsafe { allocator::ptr_to_string(out_ptr, out_size) };
    Ok(output)
}

/// Deletes a value from the store by its key.
pub fn delete(key: &str) -> Result<(), Error> {
    let (ptr, size) = unsafe { allocator::string_to_ptr(key) };
    let encoded_ptr = unsafe { host::store_delete(ptr, size) };
    let (out_ptr, out_size) = allocator::decode_encoded_ptr(encoded_ptr);
    if out_ptr == 0 || out_size == 0 {
        return Err(Error::MemoryAllocationFailed {
            context: "store_delete".to_string(),
        });
    }

    let output = unsafe { allocator::ptr_to_string(out_ptr, out_size) };
    match output.as_str() {
        DELETE_OK => Ok(()),
        _ if output.starts_with(DELETE_ERR_PREFIX) => {
            let error_message = output
                .trim_start_matches(DELETE_ERR_PREFIX)
                .trim_end_matches(')');
            Err(Error::StoreError(StoreError::FailedDelete {
                key: key.to_string(),
                reason: error_message.to_string(),
            }))
        }
        _ => Err(Error::StoreError(StoreError::UnexpectedResult)),
    }
}

/// Gets all key-value pairs from the store.
///
/// This function returns a vector of tuples, where each tuple contains a key and its corresponding
/// value at the time of the call.
///
/// There is no strict ordering guaranteed for the pairs in the result.
pub fn all() -> Result<Vec<(String, String)>, Error> {
    let encoded_ptr = unsafe { host::store_all(0, 0) };
    let (out_ptr, out_size) = allocator::decode_encoded_ptr(encoded_ptr);
    if out_ptr == 0 || out_size == 0 {
        return Err(Error::StoreError(StoreError::UnexpectedResult));
    }

    let output = crate::capnp_message_to_type!(
        out_ptr,
        out_size,
        store_capnp::store_all_response::Reader,
        types::StoreAllResults
    )?;
    Ok(output.pairs)
}

/// Deletes all key-value pairs from the store.
pub fn clear() -> Result<(), Error> {
    unsafe { host::store_clear(0, 0) };
    Ok(())
}
