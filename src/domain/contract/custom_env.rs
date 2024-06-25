use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use wasmer::Memory;

use crate::domain::contract::abort_data::AbortData;

pub struct CustomEnv {
    pub memory: Option<Memory>,
    pub abort_data: Option<AbortData>,
    pub load_function: ThreadsafeFunction<Vec<u8>, ErrorStrategy::CalleeHandled>,
}
