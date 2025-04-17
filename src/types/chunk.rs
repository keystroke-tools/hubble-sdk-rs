use crate::{capnp_get_text, entry_capnp, error};

pub struct EntryChunk {
    pub id: i32,
    pub entry_id: String,
    pub index: i32,
    pub minimum_version: i32,
    pub content: String,
    pub language: String,
    pub created_at: i64,
}

pub struct NewChunk {
    pub entry_id: String,
    pub index: i32,
    pub minimum_version: i32,
    pub content: String,
    pub language: String,
}

pub struct CreateChunksOpts {
    pub chunks: Vec<NewChunk>,
}

pub(crate) struct CreateChunksResult {
    pub count: i32,
}

impl From<entry_capnp::entry_chunk::Reader<'_>> for EntryChunk {
    fn from(value: entry_capnp::entry_chunk::Reader<'_>) -> Self {
        let entry_id = capnp_get_text!(value.get_entry_id());
        let content = capnp_get_text!(value.get_content());
        let language = capnp_get_text!(value.get_language());

        EntryChunk {
            id: value.get_id(),
            entry_id,
            index: value.get_index(),
            minimum_version: value.get_minimum_version(),
            content,
            language,
            created_at: value.get_created_at(),
        }
    }
}

impl EntryChunk {
    pub fn to_capnp_message(&self) -> Result<Vec<u8>, error::Error> {
        let mut message = capnp::message::Builder::new_default();
        let mut root = message.init_root::<entry_capnp::entry_chunk::Builder>();

        root.set_id(self.id);
        root.set_entry_id(&self.entry_id);
        root.set_index(self.index);
        root.set_minimum_version(self.minimum_version);
        root.set_content(&self.content);
        root.set_language(&self.language);

        let mut buffer = vec![];
        let mut cursor = std::io::Cursor::new(&mut buffer);
        capnp::serialize::write_message(&mut cursor, &message).map_err(error::Error::Capnp)?;

        Ok(buffer)
    }
}

impl CreateChunksOpts {
    pub fn to_capnp_message(&self) -> Result<Vec<u8>, error::Error> {
        let mut message = capnp::message::Builder::new_default();
        let root = message.init_root::<entry_capnp::create_chunks_request::Builder>();

        let mut chunks_root = root.init_chunks(self.chunks.len() as u32);
        for (i, chunk) in self.chunks.iter().enumerate() {
            let mut chunk_builder = chunks_root.reborrow().get(i as u32);
            chunk_builder.set_entry_id(&chunk.entry_id);
            chunk_builder.set_index(chunk.index);
            chunk_builder.set_minimum_version(chunk.minimum_version);
            chunk_builder.set_content(&chunk.content);
            chunk_builder.set_language(&chunk.language);
        }

        let mut buffer = vec![];
        let mut cursor = std::io::Cursor::new(&mut buffer);
        capnp::serialize::write_message(&mut cursor, &message).map_err(error::Error::Capnp)?;

        Ok(buffer)
    }
}
