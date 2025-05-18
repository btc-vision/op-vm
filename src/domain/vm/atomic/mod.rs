#[cfg(feature = "contract-threading")]
pub use self::atomic_shim::*;

#[cfg(feature = "contract-threading")]
pub use self::no_atomic_wait::*;

#[cfg(feature = "contract-threading")]
mod atomic_shim;

#[cfg(feature = "contract-threading")]
mod no_atomic_wait;
