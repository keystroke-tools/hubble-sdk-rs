pub mod allocator;
pub mod error;
pub(crate) mod host;
pub mod log;
pub mod macros;
pub mod network;
pub mod transform;
pub mod types;

pub mod entry_capnp {
    include!(concat!(env!("OUT_DIR"), "/entry_capnp.rs"));
}

pub mod store_capnp {
    include!(concat!(env!("OUT_DIR"), "/store_capnp.rs"));
}

pub mod network_capnp {
    include!(concat!(env!("OUT_DIR"), "/network_capnp.rs"));
}
