use wasmer::wasmparser::Operator;

pub fn get_gas_cost(operator: &Operator) -> u64 {
    use Operator::*;

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
        MemoryCopy { .. } => 1000,
        MemoryFill { .. } => 1000,

        BrTable { targets } => {
            2500 + 350 * targets.len() as u64
        }

        CallIndirect { .. } => {
            15000
            //u64::MAX
        }

        Try { .. } | Catch { .. } | CatchAll { .. } | Delegate { .. } | Throw { .. } | Rethrow { .. } | ThrowRef { .. } | TryTable { .. }

        | RefNull { .. } | RefIsNull { .. } | RefFunc { .. } | RefEq { .. }

        | CallRef { .. } | ReturnCallRef { .. } | RefAsNonNull { .. } | BrOnNull { .. } | BrOnNonNull { .. }

        | TypedSelect { .. } | ReturnCall { .. } | ReturnCallIndirect { .. }

        | MemoryInit { .. } | DataDrop { .. } | TableInit { .. } | ElemDrop { .. }
        | TableCopy { .. } | TableFill { .. } | TableGet { .. } | TableSet { .. } | TableGrow { .. } | TableSize { .. }

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

        | MemoryAtomicNotify { .. } | MemoryAtomicWait32 { .. } | MemoryAtomicWait64 { .. } | AtomicFence { .. } | I32AtomicLoad { .. }
        | I64AtomicLoad { .. } | I32AtomicLoad8U { .. } | I32AtomicLoad16U { .. } | I64AtomicLoad8U { .. } | I64AtomicLoad16U { .. }
        | I64AtomicLoad32U { .. } | I32AtomicStore { .. } | I64AtomicStore { .. } | I32AtomicStore8 { .. } | I32AtomicStore16 { .. }
        | I64AtomicStore8 { .. } | I64AtomicStore16 { .. } | I64AtomicStore32 { .. } | I32AtomicRmwAdd { .. } | I64AtomicRmwAdd { .. }
        | I32AtomicRmw8AddU { .. } | I32AtomicRmw16AddU { .. } | I64AtomicRmw8AddU { .. } | I64AtomicRmw16AddU { .. } | I64AtomicRmw32AddU { .. }
        | I32AtomicRmwSub { .. } | I64AtomicRmwSub { .. } | I32AtomicRmw8SubU { .. } | I32AtomicRmw16SubU { .. } | I64AtomicRmw8SubU { .. }
        | I64AtomicRmw16SubU { .. } | I64AtomicRmw32SubU { .. } | I32AtomicRmwAnd { .. } | I64AtomicRmwAnd { .. } | I32AtomicRmw8AndU { .. }
        | I32AtomicRmw16AndU { .. } | I64AtomicRmw8AndU { .. } | I64AtomicRmw16AndU { .. } | I64AtomicRmw32AndU { .. } | I32AtomicRmwOr { .. }
        | I64AtomicRmwOr { .. } | I32AtomicRmw8OrU { .. } | I32AtomicRmw16OrU { .. } | I64AtomicRmw8OrU { .. } | I64AtomicRmw16OrU { .. }
        | I64AtomicRmw32OrU { .. } | I32AtomicRmwXor { .. } | I64AtomicRmwXor { .. } | I32AtomicRmw8XorU { .. } | I32AtomicRmw16XorU { .. }
        | I64AtomicRmw8XorU { .. } | I64AtomicRmw16XorU { .. } | I64AtomicRmw32XorU { .. } | I32AtomicRmwXchg { .. } | I64AtomicRmwXchg { .. }
        | I32AtomicRmw8XchgU { .. } | I32AtomicRmw16XchgU { .. } | I64AtomicRmw8XchgU { .. } | I64AtomicRmw16XchgU { .. }
        | I64AtomicRmw32XchgU { .. } | I32AtomicRmwCmpxchg { .. } | I64AtomicRmwCmpxchg { .. } | I32AtomicRmw8CmpxchgU { .. }
        | I32AtomicRmw16CmpxchgU { .. } | I64AtomicRmw8CmpxchgU { .. } | I64AtomicRmw16CmpxchgU { .. } | I64AtomicRmw32CmpxchgU { .. }

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
    };
    gas_cost
}
