use crate::capnp_get_text;
use crate::entry_capnp;

#[derive(Default)]
pub enum Type {
    Link,
    Audio,
    Video,
    Image,
    PDF,
    Interchange,
    EPUB,
    WordDocument,
    Presentation,
    Spreadsheet,
    HTML,
    Markdown,
    PlainText,
    Archive,
    Code,
    Comment,

    #[default]
    Other,
}

#[derive(Default)]
pub enum QueueStatus {
    #[default]
    Queued,
    Processing,
    Completed,
    Failed,
    Canceled,
    Paused,
}

#[derive(Default)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Default)]
pub struct Owner {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

#[derive(Default)]
pub struct Queue {
    pub status: QueueStatus,
    pub queued_at: i64,
}

#[derive(Default)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub content: String,
    pub version: i32,
    pub r#type: Type,
    pub collection: Collection,
    pub owner: Owner,
    pub queue: Queue,
    pub created_at: i64,
    pub filesize_bytes: i64,
    /// In the case of a link, this is the link itself, otherwise it is a pre-signed Minio URL to the file.
    pub url: String,
}

impl From<entry_capnp::Status> for QueueStatus {
    fn from(value: entry_capnp::Status) -> Self {
        match value {
            entry_capnp::Status::Queued => QueueStatus::Queued,
            entry_capnp::Status::Processing => QueueStatus::Processing,
            entry_capnp::Status::Completed => QueueStatus::Completed,
            entry_capnp::Status::Failed => QueueStatus::Failed,
            entry_capnp::Status::Canceled => QueueStatus::Canceled,
            entry_capnp::Status::Paused => QueueStatus::Paused,
        }
    }
}

impl From<entry_capnp::Type> for Type {
    fn from(value: entry_capnp::Type) -> Self {
        match value {
            entry_capnp::Type::Link => Type::Link,
            entry_capnp::Type::Audio => Type::Audio,
            entry_capnp::Type::Video => Type::Video,
            entry_capnp::Type::Image => Type::Image,
            entry_capnp::Type::Pdf => Type::PDF,
            entry_capnp::Type::Interchange => Type::Interchange,
            entry_capnp::Type::Epub => Type::EPUB,
            entry_capnp::Type::WordDocument => Type::WordDocument,
            entry_capnp::Type::Presentation => Type::Presentation,
            entry_capnp::Type::Spreadsheet => Type::Spreadsheet,
            entry_capnp::Type::Html => Type::HTML,
            entry_capnp::Type::Markdown => Type::Markdown,
            entry_capnp::Type::PlainText => Type::PlainText,
            entry_capnp::Type::Archive => Type::Archive,
            entry_capnp::Type::Code => Type::Code,
            entry_capnp::Type::Comment => Type::Comment,
            entry_capnp::Type::Other => Type::Other,
        }
    }
}

impl From<entry_capnp::owner::Reader<'_>> for Owner {
    fn from(value: entry_capnp::owner::Reader<'_>) -> Self {
        let first_name = capnp_get_text!(value.get_first_name());
        let last_name = capnp_get_text!(value.get_last_name());
        let username = capnp_get_text!(value.get_username());

        Owner {
            first_name,
            last_name,
            username,
        }
    }
}

impl From<entry_capnp::collection::Reader<'_>> for Collection {
    fn from(value: entry_capnp::collection::Reader<'_>) -> Self {
        let id = capnp_get_text!(value.get_id());
        let name = capnp_get_text!(value.get_name());
        let slug = capnp_get_text!(value.get_slug());

        Collection { id, name, slug }
    }
}

impl From<entry_capnp::queue::Reader<'_>> for Queue {
    fn from(value: entry_capnp::queue::Reader<'_>) -> Self {
        let status = value.get_status().unwrap_or(entry_capnp::Status::Queued);
        let queued_at = value.get_queued_at();

        Queue {
            status: status.into(),
            queued_at,
        }
    }
}

impl From<entry_capnp::entry::Reader<'_>> for Entry {
    fn from(value: entry_capnp::entry::Reader<'_>) -> Self {
        let id = capnp_get_text!(value.get_id());
        let name = capnp_get_text!(value.get_name());
        let content = capnp_get_text!(value.get_content());
        let version = value.get_version();
        let _type = value.get_type().unwrap_or(entry_capnp::Type::Other);
        let url = capnp_get_text!(value.get_url());

        let collection = value
            .get_collection()
            .map(Collection::from)
            .unwrap_or_default();

        let owner = value.get_owner().map(Owner::from).unwrap_or_default();
        let queue = value.get_queue().map(Queue::from).unwrap_or_default();

        Entry {
            id,
            name,
            content,
            version,
            r#type: _type.into(),
            collection,
            owner,
            queue,
            created_at: value.get_created_at(),
            filesize_bytes: value.get_filesize_bytes(),
            url,
        }
    }
}

impl Entry {
    pub fn read_from_capnp(ptr: u32, len: u32) -> Result<Self, crate::error::Error> {
        let entry = crate::capnp_message_to_type!(ptr, len, entry_capnp::entry::Reader, Entry)?;
        Ok(entry)
    }
}
