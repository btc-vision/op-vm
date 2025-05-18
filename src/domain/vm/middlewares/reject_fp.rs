use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use wasmer::sys::{FunctionMiddleware, MiddlewareError, MiddlewareReaderState, ModuleMiddleware};
use wasmer::wasmparser::Operator;
use wasmer::LocalFunctionIndex;

#[derive(Default)]
struct RejectFP;

impl Debug for RejectFP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RejectFP").finish()
    }
}

impl FunctionMiddleware for RejectFP {
    fn feed<'a>(
        &mut self,
        op: Operator<'a>,
        state: &mut MiddlewareReaderState<'a>,
    ) -> Result<(), MiddlewareError> {
        if is_banned(&op) {
            return Err(MiddlewareError::new(
                "RejectFP",
                "floating-point or relaxed-SIMD op disallowed",
            ));
        }
        state.push_operator(op);
        Ok(())
    }
}

/// `true`  → reject the opcode
/// `false` → allow it to pass through
fn is_banned(op: &Operator<'_>) -> bool {
    use Operator::*;

    let scalar_fp = matches!(
        op,
        F32Abs
            | F32Neg
            | F32Ceil
            | F32Floor
            | F32Trunc
            | F32Nearest
            | F32Sqrt
            | F32Add
            | F32Sub
            | F32Mul
            | F32Div
            | F32Min
            | F32Max
            | F32Copysign
            | F64Abs
            | F64Neg
            | F64Ceil
            | F64Floor
            | F64Trunc
            | F64Nearest
            | F64Sqrt
            | F64Add
            | F64Sub
            | F64Mul
            | F64Div
            | F64Min
            | F64Max
            | F64Copysign
            | F32Eq
            | F32Ne
            | F32Lt
            | F32Gt
            | F32Le
            | F32Ge
            | F64Eq
            | F64Ne
            | F64Lt
            | F64Gt
            | F64Le
            | F64Ge
            | I32TruncF32S
            | I32TruncF32U
            | I32TruncF64S
            | I32TruncF64U
            | I64TruncF32S
            | I64TruncF32U
            | I64TruncF64S
            | I64TruncF64U
            | F32ConvertI32S
            | F32ConvertI32U
            | F32ConvertI64S
            | F32ConvertI64U
            | F64ConvertI32S
            | F64ConvertI32U
            | F64ConvertI64S
            | F64ConvertI64U
            | F32DemoteF64
            | F64PromoteF32
            | I32TruncSatF32S
            | I32TruncSatF32U
            | I32TruncSatF64S
            | I32TruncSatF64U
            | I64TruncSatF32S
            | I64TruncSatF32U
            | I64TruncSatF64S
            | I64TruncSatF64U
            | I32ReinterpretF32
            | I64ReinterpretF64
            | F32ReinterpretI32
            | F64ReinterpretI64
    );

    // relaxed-SIMD variant that currently exists
    let relaxed_simd = matches!(
        op,
        I8x16RelaxedSwizzle
            | I8x16RelaxedLaneselect
            | I16x8RelaxedLaneselect
            | I32x4RelaxedLaneselect
            | I64x2RelaxedLaneselect
            | F32x4RelaxedMin
            | F32x4RelaxedMax
            | F64x2RelaxedMin
            | F64x2RelaxedMax
            | F32x4RelaxedMadd
            | F32x4RelaxedNmadd
            | F64x2RelaxedMadd
            | F64x2RelaxedNmadd
            | I32x4RelaxedTruncF32x4S
            | I32x4RelaxedTruncF32x4U
            | I32x4RelaxedTruncF64x2SZero
            | I32x4RelaxedTruncF64x2UZero
            | I16x8RelaxedQ15mulrS
            | I16x8RelaxedDotI8x16I7x16S
            | I32x4RelaxedDotI8x16I7x16AddS
    );

    scalar_fp || relaxed_simd
}

pub struct RejectFPMiddleware;

impl RejectFPMiddleware {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}

impl Debug for RejectFPMiddleware {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RejectFPMiddleware").finish()
    }
}

impl ModuleMiddleware for RejectFPMiddleware {
    fn generate_function_middleware(&self, _: LocalFunctionIndex) -> Box<dyn FunctionMiddleware> {
        Box::<RejectFP>::default()
    }
}
