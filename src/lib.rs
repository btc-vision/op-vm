#![deny(clippy::all)]

//use crate::interfaces::create_environment_variables;
use neon::prelude::*;
use std::panic;

mod application;
mod domain;
mod interfaces;

use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

static RUNTIME: OnceCell<Runtime> = OnceCell::new();

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let runtime = RUNTIME
        .get_or_try_init(Runtime::new)
        .or_else(|err| cx.throw_error(err.to_string()))?;

    let _ = neon::set_global_executor(&mut cx, runtime);

    // Export all registered exports
    neon::registered().export(&mut cx)?;

    panic::set_hook(Box::new(|e| {
        println!("Panic occurred: {:?}", e);
    }));

    //cx.export_function("test", create_environment_variables)?;

    Ok(())
}
