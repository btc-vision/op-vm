use wasmer::wasmparser::Operator;
use wasmer::wasmparser::Operator::*;

#[cfg(feature = "contract-threading")]
const fn atomic_load_cost(bytes: u32) -> u64 {
    20_000 + (bytes as u64) * 2_000
}
#[cfg(feature = "contract-threading")]
const fn atomic_store_cost(bytes: u32) -> u64 {
    45_000 + (bytes as u64) * 3_000
}
#[cfg(feature = "contract-threading")]
const fn atomic_rmw_cost(bytes: u32) -> u64 {
    60_000 + (bytes as u64) * 4_000
}
#[cfg(feature = "contract-threading")]
const fn memory_atomic_notify() -> u64 {
    600_000
}
#[cfg(feature = "contract-threading")]
const fn memory_atomic_wait(bytes: u64) -> u64 {
    800_000 + (bytes * 6_250)
}

#[cfg(not(feature = "contract-threading"))]
const fn atomic_load_cost(_: u32) -> u64 {
    u64::MAX
}
#[cfg(not(feature = "contract-threading"))]
const fn atomic_store_cost(_: u32) -> u64 {
    u64::MAX
}
#[cfg(not(feature = "contract-threading"))]
const fn atomic_rmw_cost(_: u32) -> u64 {
    u64::MAX
}
#[cfg(not(feature = "contract-threading"))]
const fn memory_atomic_notify() -> u64 {
    u64::MAX
}
#[cfg(not(feature = "contract-threading"))]
const fn memory_atomic_wait(_: u64) -> u64 {
    u64::MAX
}

/// A near call costs 3750. An indirect call does an extra table lookup + signature check.
/// We charge a 2000-gas premium plus 500 gas for each argument/return slot so larger
/// function signatures scale naturally.
#[inline]
fn call_indirect_cost(type_arity: Option<(u32, u32)>) -> u64 {
    match type_arity {
        Some((params, results)) => 5_750 + 500 * (params + results) as u64,
        None => 7_500,
    }
}

pub const MEMORY_FILL_BASE: u64 = 50_000;
pub const MEMORY_FILL_PER_BLOCK: u64 = 30; // 16-byte block
pub const MEMORY_COPY_BASE: u64 = 60_000;
pub const MEMORY_COPY_PER_BLOCK: u64 = 60; // 16-byte block

// If we ever enable the table feature.
#[cfg(feature = "table-metering")]
pub const TABLE_FILL_BASE: u64 = 65_000;
#[cfg(feature = "table-metering")]
pub const TABLE_FILL_PER_ELEM: u64 = 30;
#[cfg(feature = "table-metering")]
pub const TABLE_COPY_BASE: u64 = 70_000;
#[cfg(feature = "table-metering")]
pub const TABLE_COPY_PER_ELEM: u64 = 50;
#[cfg(feature = "table-metering")]
pub const TABLE_GROW_BASE: u64 = 4_000;
#[cfg(feature = "table-metering")]
pub const TABLE_GROW_PER_ELEM: u64 = 500;

pub fn get_gas_cost(operator: &Operator, func_type: Option<(u32, u32)>) -> u64 {
    #[rustfmt::skip]
    let gas_cost = match operator {
        Unreachable | Return | Nop | I32Const { .. } | I64Const { .. } => 1,
        Drop => 10,
        Block { .. } | Loop { .. } | Else | End => 1,
        Br { .. } | BrIf { .. } | If { .. } => 750,
        Select { .. } => 1200,
        Call { .. } => 3750,
        LocalTee { .. } => 75,
        LocalGet { .. } => 75,
        LocalSet { .. } => 200,
        GlobalGet { .. } => 225,
        GlobalSet { .. } => 575,

        I32Eqz | I32Eq | I32Ne | I32LtS | I32LtU | I32GtS | I32GtU | I32LeS | I32LeU | I32GeS | I32GeU => 175,
        I32Clz => 250,
        I32Ctz => 2750,
        I32Add | I32Sub => 75,
        I32Mul => 150,
        I32DivS | I32DivU | I32RemS | I32RemU => 1100,
        I32And | I32Or | I32Xor | I32Shl | I32ShrS | I32ShrU | I32Rotl | I32Rotr => 75,
        I32Popcnt => 2750,
        I32Load { .. } | I32Load8S { .. } | I32Load8U { .. } | I32Load16S { .. } | I32Load16U { .. } => 675,
        I32Store { .. } | I32Store8 { .. } | I32Store16 { .. } => 850,

        I64Eqz | I64Eq | I64Ne | I64LtS | I64LtU | I64GtS | I64GtU | I64LeS | I64LeU | I64GeS | I64GeU => 225,
        I64Clz => 250,
        I64Ctz => 6000,
        I64Add | I64Sub => 100,
        I64Mul => 150,
        I64DivS | I64DivU | I64RemS | I64RemU => 1250,
        I64And | I64Or | I64Xor | I64Shl | I64ShrS | I64ShrU | I64Rotl | I64Rotr => 100,
        I64Popcnt => 6000,
        I64Load { .. } | I64Load8S { .. } | I64Load8U { .. } | I64Load16S { .. } | I64Load16U { .. } | I64Load32S { .. } | I64Load32U { .. } => 700,
        I64Store { .. } | I64Store8 { .. } | I64Store16 { .. } | I64Store32 { .. } => 1000,

        I32WrapI64 | I64ExtendI32S | I64ExtendI32U | I32Extend8S | I32Extend16S | I64Extend8S | I64Extend16S | I64Extend32S => 100,

        MemorySize { .. } => 3000,
        MemoryGrow { .. } => 8000,
    
        MemoryCopy { .. } => 0,  // Done in metering directly.
        MemoryFill { .. } => 0, // Done in metering directly.

        BrTable { targets } => {
            2500 + 350 * targets.len() as u64
        }

        CallIndirect { .. } => call_indirect_cost(func_type),

        I32AtomicStore    { .. }                      => atomic_store_cost(4),
        I32AtomicStore8   { .. }                      => atomic_store_cost(1),
        I32AtomicStore16  { .. }                      => atomic_store_cost(2),
        I64AtomicStore    { .. }                      => atomic_store_cost(8),
        I64AtomicStore8   { .. }                      => atomic_store_cost(1),
        I64AtomicStore16  { .. }                      => atomic_store_cost(2),
        I64AtomicStore32  { .. }                      => atomic_store_cost(4),

        I32AtomicLoad     { .. }                      => atomic_load_cost(4),
        I32AtomicLoad8U   { .. }                      => atomic_load_cost(1),
        I32AtomicLoad16U  { .. }                      => atomic_load_cost(2),
        I64AtomicLoad     { .. }                      => atomic_load_cost(8),
        I64AtomicLoad8U   { .. }                      => atomic_load_cost(1),
        I64AtomicLoad16U  { .. }                      => atomic_load_cost(2),
        I64AtomicLoad32U  { .. }                      => atomic_load_cost(4),

        I32AtomicRmwAdd { .. }          |
        I32AtomicRmwSub { .. }          |
        I32AtomicRmwAnd { .. }          |
        I32AtomicRmwOr  { .. }          |
        I32AtomicRmwXor { .. }          |
        I32AtomicRmwXchg { .. }         |
        I32AtomicRmwCmpxchg { .. }      => atomic_rmw_cost(4),

        I32AtomicRmw8AddU { .. }        |
        I32AtomicRmw8SubU { .. }        |
        I32AtomicRmw8AndU { .. }        |
        I32AtomicRmw8OrU  { .. }        |
        I32AtomicRmw8XorU { .. }        |
        I32AtomicRmw8XchgU { .. }       |
        I32AtomicRmw8CmpxchgU { .. }    => atomic_rmw_cost(1),

        I32AtomicRmw16AddU { .. }       |
        I32AtomicRmw16SubU { .. }       |
        I32AtomicRmw16AndU { .. }       |
        I32AtomicRmw16OrU  { .. }       |
        I32AtomicRmw16XorU { .. }       |
        I32AtomicRmw16XchgU { .. }      |
        I32AtomicRmw16CmpxchgU { .. }   => atomic_rmw_cost(2),

        I64AtomicRmwAdd { .. }          |
        I64AtomicRmwSub { .. }          |
        I64AtomicRmwAnd { .. }          |
        I64AtomicRmwOr  { .. }          |
        I64AtomicRmwXor { .. }          |
        I64AtomicRmwXchg { .. }         |
        I64AtomicRmwCmpxchg { .. }      => atomic_rmw_cost(8),

        I64AtomicRmw8AddU { .. }        |
        I64AtomicRmw8SubU { .. }        |
        I64AtomicRmw8AndU { .. }        |
        I64AtomicRmw8OrU  { .. }        |
        I64AtomicRmw8XorU { .. }        |
        I64AtomicRmw8XchgU { .. }       |
        I64AtomicRmw8CmpxchgU { .. }    => atomic_rmw_cost(1),

        I64AtomicRmw16AddU { .. }       |
        I64AtomicRmw16SubU { .. }       |
        I64AtomicRmw16AndU { .. }       |
        I64AtomicRmw16OrU  { .. }       |
        I64AtomicRmw16XorU { .. }       |
        I64AtomicRmw16XchgU { .. }      |
        I64AtomicRmw16CmpxchgU { .. }   => atomic_rmw_cost(2),

        I64AtomicRmw32AddU { .. }       |
        I64AtomicRmw32SubU { .. }       |
        I64AtomicRmw32AndU { .. }       |
        I64AtomicRmw32OrU  { .. }       |
        I64AtomicRmw32XorU { .. }       |
        I64AtomicRmw32XchgU { .. }      |
        I64AtomicRmw32CmpxchgU { .. }   => atomic_rmw_cost(4),

        MemoryAtomicNotify { .. }       => memory_atomic_notify(),
        MemoryAtomicWait32 { .. }       => memory_atomic_wait(32),
        MemoryAtomicWait64 { .. }       => memory_atomic_wait(64),

        //----------------------------------------------------------------
        //  TABLE  – only compiled when the feature is active
        //----------------------------------------------------------------
        #[cfg(feature = "table-metering")]
        TableCopy { .. } => 0,  // Done in metering directly.
        #[cfg(not(feature = "table-metering"))]
        TableCopy { .. } => u64::MAX,

        #[cfg(feature = "table-metering")]
        TableFill { .. } => 0,  // Done in metering directly.
        #[cfg(not(feature = "table-metering"))]
        TableFill { .. } => u64::MAX,
    
        #[cfg(feature = "table-metering")]
        TableGrow { .. } => 0,  // Done in metering directly.
        #[cfg(not(feature = "table-metering"))]
        TableGrow { .. } => u64::MAX,
    
        #[cfg(feature = "table-metering")]
        TableInit { .. } => 0,  // Done in metering directly.
        #[cfg(not(feature = "table-metering"))]
        TableInit { .. } => u64::MAX,

        #[cfg(feature = "table-metering")]
        TableGet { .. }  => 900,
        #[cfg(not(feature = "table-metering"))]
        TableGet { .. }  => u64::MAX,

        #[cfg(feature = "table-metering")]
        TableSet { .. }  => 1_100,
        #[cfg(not(feature = "table-metering"))]
        TableSet { .. }  => u64::MAX,

        #[cfg(feature = "table-metering")]
        TableSize { .. } => 2_500,
        #[cfg(not(feature = "table-metering"))]
        TableSize { .. } => u64::MAX,
    
        MemoryInit { .. } |

        Try { .. } | Catch { .. } | CatchAll { .. } | Delegate { .. } | Throw { .. } | Rethrow { .. } | ThrowRef { .. } | TryTable { .. }

        | RefNull { .. } | RefIsNull { .. } | RefFunc { .. } | RefEq { .. }

        | CallRef { .. } | ReturnCallRef { .. } | RefAsNonNull { .. } | BrOnNull { .. } | BrOnNonNull { .. }

        | TypedSelect { .. } | ReturnCall { .. } | ReturnCallIndirect { .. }

        | DataDrop { .. } | ElemDrop { .. }

        | MemoryDiscard { .. }

        | StructNew { .. } | StructNewDefault { .. } | StructGet { .. } | StructGetS { .. } | StructGetU { .. } | StructSet { .. }
        | ArrayNew { .. } | ArrayNewDefault { .. } | ArrayNewFixed { .. } | ArrayNewData { .. } | ArrayNewElem { .. }
        | ArrayGet { .. } | ArrayGetS { .. } | ArrayGetU { .. } | ArraySet { .. } | ArrayLen { .. } | ArrayFill { .. } | ArrayCopy { .. }
        | ArrayInitData { .. } | ArrayInitElem { .. } | RefTestNonNull { .. } | RefTestNullable { .. } | RefCastNonNull { .. } | RefCastNullable { .. }
        | BrOnCast { .. } | BrOnCastFail { .. } | AnyConvertExtern { .. } | ExternConvertAny { .. } | RefI31 { .. } | I31GetS { .. } | I31GetU { .. }

        | F32Load { .. } | F64Load { .. } | F32Store { .. } | F64Store { .. } | F32Const { .. } | F64Const { .. }
        | F32Eq { .. } | F32Ne { .. } | F32Lt { .. } | F32Gt { .. } | F32Le { .. } | F32Ge { .. }
        | F64Eq { .. } | F64Ne { .. } | F64Lt { .. } | F64Gt { .. } | F64Le { .. } | F64Ge { .. }
        | F32Abs { .. } | F32Neg { .. } | F32Ceil { .. } | F32Floor { .. } | F32Trunc { .. } | F32Nearest { .. } | F32Sqrt { .. } | F32Add { .. } | F32Sub { .. } | F32Mul { .. }
        | F32Div { .. } | F32Min { .. } | F32Max { .. } | F32Copysign { .. } | F64Abs { .. } | F64Neg { .. } | F64Ceil { .. } | F64Floor { .. } | F64Trunc { .. }
        | F64Nearest { .. } | F64Sqrt { .. } | F64Add { .. } | F64Sub { .. } | F64Mul { .. } | F64Div { .. } | F64Min { .. } | F64Max { .. } | F64Copysign { .. }
        | I32TruncF32S { .. } | I32TruncF32U { .. } | I32TruncF64S { .. } | I32TruncF64U { .. }
        | I64TruncF32S { .. } | I64TruncF32U { .. } | I64TruncF64S { .. } | I64TruncF64U { .. }
        | F32ConvertI32S { .. } | F32ConvertI32U { .. } | F32ConvertI64S { .. } | F32ConvertI64U { .. } | F32DemoteF64 { .. }
        | F64ConvertI32S { .. } | F64ConvertI32U { .. } | F64ConvertI64S { .. } | F64ConvertI64U { .. } | F64PromoteF32 { .. }
        | I32ReinterpretF32 { .. } | I64ReinterpretF64 { .. } | F32ReinterpretI32 { .. } | F64ReinterpretI64 { .. }
        | I32TruncSatF32S { .. } | I32TruncSatF32U { .. } | I32TruncSatF64S { .. } | I32TruncSatF64U { .. }
        | I64TruncSatF32S { .. } | I64TruncSatF32U { .. } | I64TruncSatF64S { .. } | I64TruncSatF64U { .. }

        | AtomicFence { .. }

        | V128Load { .. } | V128Load8x8S { .. } | V128Load8x8U { .. } | V128Load16x4S { .. } | V128Load16x4U { .. } | V128Load32x2S { .. } | V128Load32x2U { .. }
        | V128Load8Splat { .. } | V128Load16Splat { .. } | V128Load32Splat { .. } | V128Load64Splat { .. } | V128Load32Zero { .. } | V128Load64Zero { .. }
        | V128Store { .. } | V128Load8Lane { .. } | V128Load16Lane { .. } | V128Load32Lane { .. } | V128Load64Lane { .. } | V128Store8Lane { .. }
        | V128Store16Lane { .. } | V128Store32Lane { .. } | V128Store64Lane { .. } | V128Const { .. }
        | I8x16Shuffle { .. } | I8x16ExtractLaneS { .. } | I8x16ExtractLaneU { .. } | I8x16ReplaceLane { .. } | I16x8ExtractLaneS { .. }
        | I16x8ExtractLaneU { .. } | I16x8ReplaceLane { .. } | I32x4ExtractLane { .. } | I32x4ReplaceLane { .. } | I64x2ExtractLane { .. }
        | I64x2ReplaceLane { .. } | F32x4ExtractLane { .. } | F32x4ReplaceLane { .. } | F64x2ExtractLane { .. } | F64x2ReplaceLane { .. }
        | I8x16Swizzle { .. } | I8x16Splat { .. } | I16x8Splat { .. } | I32x4Splat { .. } | I64x2Splat { .. } | F32x4Splat { .. } | F64x2Splat { .. } | I8x16Eq { .. }
        | I8x16Ne { .. } | I8x16LtS { .. } | I8x16LtU { .. } | I8x16GtS { .. } | I8x16GtU { .. } | I8x16LeS { .. } | I8x16LeU { .. } | I8x16GeS { .. } | I8x16GeU { .. } | I16x8Eq { .. }
        | I16x8Ne { .. } | I16x8LtS { .. } | I16x8LtU { .. } | I16x8GtS { .. } | I16x8GtU { .. } | I16x8LeS { .. } | I16x8LeU { .. } | I16x8GeS { .. } | I16x8GeU { .. } | I32x4Eq { .. }
        | I32x4Ne { .. } | I32x4LtS { .. } | I32x4LtU { .. } | I32x4GtS { .. } | I32x4GtU { .. } | I32x4LeS { .. } | I32x4LeU { .. } | I32x4GeS { .. } | I32x4GeU { .. } | I64x2Eq { .. }
        | I64x2Ne { .. } | I64x2LtS { .. } | I64x2GtS { .. } | I64x2LeS { .. } | I64x2GeS { .. }
        | F32x4Eq { .. } | F32x4Ne { .. } | F32x4Lt { .. } | F32x4Gt { .. } | F32x4Le { .. } | F32x4Ge { .. }
        | F64x2Eq { .. } | F64x2Ne { .. } | F64x2Lt { .. } | F64x2Gt { .. } | F64x2Le { .. } | F64x2Ge { .. }
        | V128Not { .. } | V128And { .. } | V128AndNot { .. } | V128Or { .. } | V128Xor { .. } | V128Bitselect { .. } | V128AnyTrue { .. }
        | I8x16Abs { .. } | I8x16Neg { .. } | I8x16Popcnt { .. } | I8x16AllTrue { .. } | I8x16Bitmask { .. } | I8x16NarrowI16x8S { .. } | I8x16NarrowI16x8U { .. }
        | I8x16Shl { .. } | I8x16ShrS { .. } | I8x16ShrU { .. } | I8x16Add { .. } | I8x16AddSatS { .. } | I8x16AddSatU { .. } | I8x16Sub { .. } | I8x16SubSatS { .. }
        | I8x16SubSatU { .. } | I8x16MinS { .. } | I8x16MinU { .. } | I8x16MaxS { .. } | I8x16MaxU { .. } | I8x16AvgrU { .. }
        | I16x8ExtAddPairwiseI8x16S { .. } | I16x8ExtAddPairwiseI8x16U { .. } | I16x8Abs { .. } | I16x8Neg { .. } | I16x8Q15MulrSatS { .. }
        | I16x8AllTrue { .. } | I16x8Bitmask { .. } | I16x8NarrowI32x4S { .. } | I16x8NarrowI32x4U { .. } | I16x8ExtendLowI8x16S { .. }
        | I16x8ExtendHighI8x16S { .. } | I16x8ExtendLowI8x16U { .. } | I16x8ExtendHighI8x16U { .. } | I16x8Shl { .. } | I16x8ShrS { .. } | I16x8ShrU { .. }
        | I16x8Add { .. } | I16x8AddSatS { .. } | I16x8AddSatU { .. } | I16x8Sub { .. } | I16x8SubSatS { .. } | I16x8SubSatU { .. } | I16x8Mul { .. } | I16x8MinS { .. }
        | I16x8MinU { .. } | I16x8MaxS { .. } | I16x8MaxU { .. } | I16x8AvgrU { .. } | I16x8ExtMulLowI8x16S { .. }
        | I16x8ExtMulHighI8x16S { .. } | I16x8ExtMulLowI8x16U { .. } | I16x8ExtMulHighI8x16U { .. } | I32x4ExtAddPairwiseI16x8S { .. }
        | I32x4ExtAddPairwiseI16x8U { .. } | I32x4Abs { .. } | I32x4Neg { .. } | I32x4AllTrue { .. } | I32x4Bitmask { .. } | I32x4ExtendLowI16x8S { .. }
        | I32x4ExtendHighI16x8S { .. } | I32x4ExtendLowI16x8U { .. } | I32x4ExtendHighI16x8U { .. } | I32x4Shl { .. } | I32x4ShrS { .. } | I32x4ShrU { .. }
        | I32x4Add { .. } | I32x4Sub { .. } | I32x4Mul { .. } | I32x4MinS { .. } | I32x4MinU { .. } | I32x4MaxS { .. } | I32x4MaxU { .. } | I32x4DotI16x8S { .. }
        | I32x4ExtMulLowI16x8S { .. } | I32x4ExtMulHighI16x8S { .. } | I32x4ExtMulLowI16x8U { .. } | I32x4ExtMulHighI16x8U { .. } | I64x2Abs { .. }
        | I64x2Neg { .. } | I64x2AllTrue { .. } | I64x2Bitmask { .. } | I64x2ExtendLowI32x4S { .. } | I64x2ExtendHighI32x4S { .. }
        | I64x2ExtendLowI32x4U { .. } | I64x2ExtendHighI32x4U { .. } | I64x2Shl { .. } | I64x2ShrS { .. } | I64x2ShrU { .. } | I64x2Add { .. } | I64x2Sub { .. }
        | I64x2Mul { .. } | I64x2ExtMulLowI32x4S { .. } | I64x2ExtMulHighI32x4S { .. } | I64x2ExtMulLowI32x4U { .. } | I64x2ExtMulHighI32x4U { .. }
        | F32x4Ceil { .. } | F32x4Floor { .. } | F32x4Trunc { .. } | F32x4Nearest { .. } | F32x4Abs { .. } | F32x4Neg { .. } | F32x4Sqrt { .. } | F32x4Add { .. } | F32x4Sub { .. }
        | F32x4Mul { .. } | F32x4Div { .. } | F32x4Min { .. } | F32x4Max { .. } | F32x4PMin { .. } | F32x4PMax { .. } | F64x2Ceil { .. } | F64x2Floor { .. } | F64x2Trunc { .. }
        | F64x2Nearest { .. } | F64x2Abs { .. } | F64x2Neg { .. } | F64x2Sqrt { .. } | F64x2Add { .. } | F64x2Sub { .. } | F64x2Mul { .. } | F64x2Div { .. } | F64x2Min { .. }
        | F64x2Max { .. } | F64x2PMin { .. } | F64x2PMax { .. } | I32x4TruncSatF32x4S { .. } | I32x4TruncSatF32x4U { .. } | F32x4ConvertI32x4S { .. }
        | F32x4ConvertI32x4U { .. } | I32x4TruncSatF64x2SZero { .. } | I32x4TruncSatF64x2UZero { .. } | F64x2ConvertLowI32x4S { .. }
        | F64x2ConvertLowI32x4U { .. } | F32x4DemoteF64x2Zero { .. } | F64x2PromoteLowF32x4 { .. } | I8x16RelaxedSwizzle { .. }
        | I32x4RelaxedTruncF32x4S { .. } | I32x4RelaxedTruncF32x4U { .. } | I32x4RelaxedTruncF64x2SZero { .. }
        | I32x4RelaxedTruncF64x2UZero { .. } | F32x4RelaxedMadd { .. } | F32x4RelaxedNmadd { .. } | F64x2RelaxedMadd { .. }
        | F64x2RelaxedNmadd { .. } | I8x16RelaxedLaneselect { .. } | I16x8RelaxedLaneselect { .. } | I32x4RelaxedLaneselect { .. }
        | I64x2RelaxedLaneselect { .. } | F32x4RelaxedMin { .. } | F32x4RelaxedMax { .. } | F64x2RelaxedMin { .. } | F64x2RelaxedMax { .. }
        | I16x8RelaxedQ15mulrS { .. } | I16x8RelaxedDotI8x16I7x16S { .. } | I32x4RelaxedDotI8x16I7x16AddS { .. }
        => {
            u64::MAX
        }
    _ => {
        u64::MAX
    }};

    gas_cost
}
