use crate::{allocator, error::Error, host, read_chunk_result};

/// Chunks the input string into smaller pieces with overlap (to retain context).
/// This is useful for tasks like summarization or question answering.
pub fn chunk_with_overlap(s: &str) -> Result<Vec<String>, Error> {
    let (ptr, size) = unsafe { allocator::string_to_ptr(s) };
    let chunks = unsafe { host::chunk_with_overlap(ptr, size) };

    let (out_ptr, out_size) = allocator::read_ptr_len(chunks);

    read_chunk_result!(out_ptr, out_size)
}

/// Chunks the input string into smaller pieces by sentence.
pub fn chunk_by_sentence(s: &str) -> Result<Vec<String>, Error> {
    let (ptr, size) = unsafe { allocator::string_to_ptr(s) };
    let chunks = unsafe { host::chunk_by_sentence(ptr, size) };

    let (out_ptr, out_size) = allocator::read_ptr_len(chunks);

    read_chunk_result!(out_ptr, out_size)
}

/// Fetches the content of a URL and converts it to Markdown format.
pub fn url_to_markdown(url: &str) -> Result<String, Error> {
    let (ptr, size) = unsafe { allocator::string_to_ptr(url) };
    let result = unsafe { host::transform_url_to_markdown(ptr, size) };

    let (out_ptr, out_size) = allocator::read_ptr_len(result);
    if out_ptr == 0 || out_size == 0 {
        return Err(Error::MemoryAllocationFailed);
    }

    let output = unsafe { allocator::ptr_to_string(out_ptr, out_size) };
    if output.is_empty() {
        return Err(Error::EmptyStringError);
    }

    Ok(output)
}

/// Converts HTML content to Markdown format.
/// This is useful for processing raw web pages or other HTML documents.
pub fn html_to_markdown(html: &str) -> Result<String, Error> {
    let (ptr, size) = unsafe { allocator::string_to_ptr(html) };
    let result = unsafe { host::transform_html_to_markdown(ptr, size) };

    let (out_ptr, out_size) = allocator::read_ptr_len(result);
    if out_ptr == 0 || out_size == 0 {
        return Err(Error::MemoryAllocationFailed);
    }

    let output = unsafe { allocator::ptr_to_string(out_ptr, out_size) };
    if output.is_empty() {
        return Err(Error::EmptyStringError);
    }

    Ok(output)
}
