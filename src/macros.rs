#[macro_export]
macro_rules! capnp_str {
    ($expr:expr) => {
        $expr
            .map_err(error::Error::Capnp)?
            .to_str()
            .map_err(error::Error::Utf8)?
    };
}

#[macro_export]
macro_rules! capnp_get_text {
    ($fn:expr) => {
        $fn.ok()
            .and_then(|s| s.to_str().ok())
            .unwrap_or_default()
            .to_string()
    };
}

#[macro_export]
macro_rules! read_chunk_result {
    ($ptr:expr, $size:expr) => {{
        use $crate::error::Error;

        let (ptr, size) = ($ptr as *const u8, $size as usize);
        let slice = unsafe { core::slice::from_raw_parts(ptr, size) };
        let mut cursor = std::io::Cursor::new(slice);
        let message =
            capnp::serialize::read_message(&mut cursor, capnp::message::ReaderOptions::new())
                .map_err(Error::Capnp)?;

        let chunk_result = message
            .get_root::<entry_capnp::chunk_result::Reader>()
            .map_err(Error::Capnp)?;

        let chunks_reader = chunk_result.get_chunks().map_err(Error::Capnp)?;

        let mut chunks = Vec::new();
        for i in 0..chunks_reader.len() {
            let chunk = capnp_str!(chunks_reader.get(i));
            chunks.push(chunk.to_string());
        }

        Ok(chunks)
    }};
}

#[macro_export]
macro_rules! capnp_message_to_type {
    ($ptr:expr, $size:expr, $reader_type:ty, $rust_type:ty) => {{
        use capnp::{message::ReaderOptions, serialize};

        // Get pointer and size first, then enter unsafe context
        let ptr = $ptr as *const u8;
        let size = $size as usize;

        // SAFETY: Caller must ensure ptr and len are valid
        let slice = unsafe { core::slice::from_raw_parts(ptr, size) };
        let mut cursor = std::io::Cursor::new(slice);

        let message = serialize::read_message(&mut cursor, ReaderOptions::new())
            .map_err($crate::error::Error::Capnp)?;

        let root = message
            .get_root::<$reader_type>()
            .map_err($crate::error::Error::Capnp)?;

        Ok(<$rust_type>::from(root))
    }};
}
