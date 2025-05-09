use std::fmt::Display;

use crate::error::Error;

#[derive(Debug, Clone, Copy, Default)]
pub enum RandSize {
    /// 16 bytes
    Small,
    /// 32 bytes (default)
    #[default]
    Medium,
    /// 64 bytes
    Large,
    /// Custom size
    Custom(u32),
}

impl From<RandSize> for u32 {
    fn from(val: RandSize) -> Self {
        match val {
            RandSize::Small => 16,
            RandSize::Medium => 32,
            RandSize::Large => 64,
            RandSize::Custom(size) => size,
        }
    }
}

impl Display for RandSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size_str = match self {
            RandSize::Small => "Small",
            RandSize::Medium => "Medium",
            RandSize::Large => "Large",
            RandSize::Custom(size) => &format!("Custom({})", size),
        };
        write!(f, "{}", size_str)
    }
}

/// Generates random bytes of the specified size.
pub fn rand(size: RandSize) -> Result<Vec<u8>, Error> {
    let rand_size: u32 = size.into();
    let encoded = unsafe { crate::host::crypto_rand(rand_size, 0) }; // We wil just write the size
    // directly to the host

    let (out_ptr, out_size) = crate::allocator::decode_encoded_ptr("rand", encoded)?;
    let buf = unsafe { crate::allocator::ptr_to_buffer(out_ptr, out_size) };

    if buf.len() != rand_size as usize {
        return Err(Error::BadRandomSize {
            expected: rand_size,
            actual: buf.len() as u32,
        });
    }

    Ok(buf)
}
