#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[macro_use]
mod application;
#[macro_use]
mod domain;
#[macro_use]
mod interfaces;

#[napi]
pub fn init() {
    //panic::set_hook(Box::new(|_| {}));
}
