use std::sync::Arc;

use crate::domain::contract::abort_data::AbortData;

pub struct CustomEnv {
    pub abort_data: Option<AbortData>,
    pub load_function: Arc<dyn Fn(u32) -> u32 + Sync + Send>,
}
