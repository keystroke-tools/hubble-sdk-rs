#[link(wasm_import_module = "env")]
unsafe extern "C" {
    #[link_name = "debug"]
    pub(crate) fn debug(ptr: u32, size: u32) -> u64;

    #[link_name = "error"]
    pub(crate) fn error(ptr: u32, size: u32) -> u64;

    #[link_name = "chunk_with_overlap"]
    pub(crate) fn chunk_with_overlap(ptr: u32, size: u32) -> u64;

    #[link_name = "chunk_by_sentence"]
    pub(crate) fn chunk_by_sentence(ptr: u32, size: u32) -> u64;

    #[link_name = "request"]
    pub(crate) fn network_request(ptr: u32, size: u32) -> u64;

    #[link_name = "transform_url_to_markdown"]
    pub(crate) fn transform_url_to_markdown(ptr: u32, size: u32) -> u64;

    #[link_name = "transform_html_to_markdown"]
    pub(crate) fn transform_html_to_markdown(ptr: u32, size: u32) -> u64;
}
