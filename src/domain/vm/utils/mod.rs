#[cfg(feature = "use-strings-instead-of-buffers")]
pub use self::bytes_to_hex::*;

#[cfg(feature = "use-strings-instead-of-buffers")]
mod bytes_to_hex;
