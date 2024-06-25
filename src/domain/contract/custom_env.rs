use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use wasmer::Memory;

use crate::domain::contract::abort_data::AbortData;

pub struct CustomEnv {
    pub abort_data: Option<AbortData>,
    pub memory: Option<Memory>,
    pub load_function: ThreadsafeFunction<u32, ErrorStrategy::CalleeHandled>,
}
