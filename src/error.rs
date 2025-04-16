#[derive(Debug)]
pub enum Error {
    Capnp(capnp::Error),
    Utf8(std::str::Utf8Error),
    Io(std::io::Error),
    PluginError(String),
    MemoryError(String),
    ReadMemoryError(String),
    MemoryAllocationFailed,
}
