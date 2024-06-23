use std::sync::Arc;

use napi::bindgen_prelude::BigInt;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};

use crate::domain::contract::abort_data::AbortData;

pub struct CustomEnv {
    pub abort_data: Option<AbortData>,
    pub load_function: Arc<ThreadsafeFunction<BigInt, ErrorStrategy::Fatal>>,
    pub store_function: Arc<ThreadsafeFunction<BigInt, ErrorStrategy::Fatal>>,
}
