#![cfg(feature = "use-strings-instead-of-buffers")]
use wasmer::RuntimeError;

/// Convert a `Vec<u8>` into a lowercase hexadecimal `String`.
pub fn vec_to_hex(bytes: &[u8]) -> String {
    bytes.into_iter().map(|b| format!("{:02x}", b)).collect()
}

/// Convert a **clean** (no `0x`, only 0-9a-fA-F) hex `String` back into `Vec<u8>`.
///
/// Returns `Err(&'static str)` on an odd length or invalid hex digit.
pub fn hex_to_vec(hex: String) -> Result<Vec<u8>, RuntimeError> {
    let len = hex.len();
    if len % 2 != 0 {
        return Err(RuntimeError::new("Invalid hex string"));
    }

    // Pre-allocate for speed
    let mut out = Vec::with_capacity(len / 2);

    // Walk the owned string in 2-char chunks
    for i in (0..len).step_by(2) {
        let byte = u8::from_str_radix(&hex[i..i + 2], 16)
            .map_err(|_| RuntimeError::new("invalid hex digit"))?;

        out.push(byte);
    }
    Ok(out)
}
