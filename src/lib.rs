#![deny(clippy::all)]
#![cfg_attr(
    all(feature = "napi", not(target_arch = "wasm32")),
    napi::bindgen_prelude::napi
)]

#[macro_use]
extern crate napi_derive;

mod application;
mod domain;
mod interfaces;

#[napi]
pub fn init() {
    //panic::set_hook(Box::new(|_| {}));
}
