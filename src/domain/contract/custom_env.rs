use crate::domain::contract::abort_data::AbortData;

pub struct CustomEnv {
    pub abort_data: Option<AbortData>,
}
