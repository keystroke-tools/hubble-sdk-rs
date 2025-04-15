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
macro_rules! read_chunk_result {
    ($ptr:expr, $size:expr) => {{
        use $crate::error::Error;

        let slice = unsafe { core::slice::from_raw_parts($ptr as *const u8, $size as usize) };
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
