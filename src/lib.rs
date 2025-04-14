#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::panic;

mod application;
mod domain;
mod interfaces;

#[napi]
pub fn init() {
    panic::set_hook(Box::new(|e| {
        println!("Panic occurred: {:?}", e);
    }));

    debug_assert!(true);
}
