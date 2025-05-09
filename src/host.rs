#[link(wasm_import_module = "env")]
unsafe extern "C" {
    #[link_name = "log_debug"]
    pub(crate) fn debug(ptr: u32, size: u32) -> u64;

    #[link_name = "log_error"]
    pub(crate) fn error(ptr: u32, size: u32) -> u64;

    #[link_name = "log_warn"]
    pub(crate) fn warn(ptr: u32, size: u32) -> u64;

    #[link_name = "chunk_with_overlap"]
    pub(crate) fn chunk_with_overlap(ptr: u32, size: u32) -> u64;

    #[link_name = "chunk_by_sentence"]
    pub(crate) fn chunk_by_sentence(ptr: u32, size: u32) -> u64;

    #[link_name = "network_request"]
    pub(crate) fn network_request(ptr: u32, size: u32) -> u64;

    #[link_name = "transform_url_to_markdown"]
    pub(crate) fn transform_url_to_markdown(ptr: u32, size: u32) -> u64;

    #[link_name = "transform_html_to_markdown"]
    pub(crate) fn transform_html_to_markdown(ptr: u32, size: u32) -> u64;

    #[link_name = "entry_update"]
    pub(crate) fn entry_update(ptr: u32, size: u32) -> u64;

    #[link_name = "entry_create_chunks"]
    pub(crate) fn entry_create_chunks(ptr: u32, size: u32) -> u64;

    // Store
    #[link_name = "store_get"]
    pub(crate) fn store_get(ptr: u32, size: u32) -> u64;

    #[link_name = "store_set"]
    pub(crate) fn store_set(ptr: u32, size: u32) -> u64;

    #[link_name = "store_delete"]
    pub(crate) fn store_delete(ptr: u32, size: u32) -> u64;

    #[link_name = "store_all"]
    pub(crate) fn store_all(ptr: u32, size: u32) -> u64;

    #[link_name = "store_clear"]
    pub(crate) fn store_clear(ptr: u32, size: u32) -> u64;

    // Crypto
    #[link_name = "crypto_rand"]
    pub(crate) fn crypto_rand(ptr: u32, size: u32) -> u64;
}
