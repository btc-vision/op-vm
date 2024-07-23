#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::panic;

mod domain;
mod interfaces;
mod application;

#[napi]
pub fn init() {
    panic::set_hook(Box::new(|_| {}));
}
