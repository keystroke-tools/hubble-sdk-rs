pub mod allocator;
pub mod entry;
pub mod error;
pub(crate) mod host;
pub mod log;
pub mod macros;
pub mod network;
pub mod store;
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

/// Generates a SHA-256 checksum for the given data.
pub fn generate_checksum(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    format!("{:x}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_checksum() {
        let data = b"Hello, world!";
        let checksum = generate_checksum(data);
        assert_eq!(
            checksum,
            "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3"
        );
    }
}
