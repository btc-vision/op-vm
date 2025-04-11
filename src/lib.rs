#![deny(clippy::all)]

use crate::interfaces::create_environment_variables;
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

    cx.export_function("test", create_environment_variables)?;

    Ok(())
}
