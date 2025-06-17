#[cfg(feature = "contract-threading")]
pub use self::ticket_futex::*;

#[cfg(feature = "contract-threading")]
mod ticket_futex;
