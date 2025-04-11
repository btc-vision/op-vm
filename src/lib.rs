#![deny(clippy::all)]

use neon::prelude::*;
use std::panic;

mod application;
mod domain;
mod interfaces;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    panic::set_hook(Box::new(|e| {
        println!("Panic occurred: {:?}", e);
    }));

    Ok(())
}
