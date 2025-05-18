#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::panic;

#[cfg(all(
    feature = "contract-threading",
    not(any(feature = "vdf", feature = "vdf-zk-snark"))
))]
compile_error!("feature \"contract-threading\" requires either \"vdf\" or \"vdf-zk-snark\"");

mod application;
mod domain;
mod interfaces;

#[napi]
pub fn init() {
    #[cfg(feature = "debug-metering")]
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    #[cfg(feature = "debug-metering")]
    log::set_max_level(log::LevelFilter::Trace);

    panic::set_hook(Box::new(|e| {
        println!("Panic occurred: {:?}", e);
    }));

    debug_assert!(true);
}
