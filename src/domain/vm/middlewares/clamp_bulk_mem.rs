use std::fmt::Debug;
use std::sync::Arc;
use wasmer::sys::{FunctionMiddleware, MiddlewareError, MiddlewareReaderState, ModuleMiddleware};
use wasmer::wasmparser::{BlockType, Operator};
use wasmer::LocalFunctionIndex;

#[derive(Debug)]
pub struct ClampBulkMem {
    pub max_len: u32,
}

impl ClampBulkMem {
    pub fn new(max_len: u32) -> Arc<Self> {
        Arc::new(Self { max_len })
    }
}

#[derive(Debug)]
struct FunctionClamp {
    max_len: u32,
    last_local_get: Option<u32>,
}

impl FunctionMiddleware for FunctionClamp {
    fn feed<'a>(
        &mut self,
        op: Operator<'a>,
        state: &mut MiddlewareReaderState<'a>,
    ) -> Result<(), MiddlewareError> {
        use Operator::*;

        if let LocalGet { local_index } = op {
            self.last_local_get = Some(local_index);
        }

        match op {
            MemoryCopy { .. } | MemoryFill { .. } | TableCopy { .. } => {
                let len_local = self.last_local_get.ok_or_else(|| {
                    MiddlewareError::new(
                        "ClampBulkMem",
                        "bulk-memory op must be preceded by `local.get` (len)",
                    )
                })?;

                state.extend(&[
                    LocalTee {
                        local_index: len_local,
                    },
                    LocalGet {
                        local_index: len_local,
                    },
                    I32Const {
                        value: self.max_len as i32,
                    },
                    I32GtU,
                    If {
                        blockty: BlockType::Empty,
                    },
                    Unreachable,
                    End,
                    op,
                ]);
            }
            _ => state.push_operator(op),
        }
        Ok(())
    }
}

impl ModuleMiddleware for ClampBulkMem {
    fn generate_function_middleware(&self, _: LocalFunctionIndex) -> Box<dyn FunctionMiddleware> {
        Box::new(FunctionClamp {
            max_len: self.max_len,
            last_local_get: None,
        })
    }
}
