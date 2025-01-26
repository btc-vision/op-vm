use crate::domain::runner::{AbortData, ExtendedMemoryAccessError};
use bytes::Bytes;
use std::sync::mpsc;
use wasmer::Value;
use wasmer_types::SerializeError;

// `RunnerResponse` enumerates all possible results we can get from the worker thread:
//#[derive(Debug)]
pub enum RunnerResponse {
    SerializeResult(Result<Bytes, SerializeError>),
    CallResult(anyhow::Result<Box<[Value]>>),
    ReadMemoryResult(Result<Vec<u8>, ExtendedMemoryAccessError>),
    WriteMemoryResult(Result<(), ExtendedMemoryAccessError>),
    WriteBufferResult(Result<i64, napi::Error>),
    RemainingGas(u64),
    OutOfMemory(Result<bool, ExtendedMemoryAccessError>),
    AbortData(Option<AbortData>),
}

// `RunnerCommand` enumerates all possible actions we can ask our WasmerRunner worker to do.
pub enum RunnerCommand {
    Serialize {
        reply_to: mpsc::Sender<RunnerResponse>,
    },
    Call {
        function: String,
        params: Vec<Value>,
        reply_to: mpsc::Sender<RunnerResponse>,
    },
    ReadMemory {
        offset: u64,
        length: u64,
        reply_to: mpsc::Sender<RunnerResponse>,
    },
    WriteMemory {
        offset: u64,
        data: Vec<u8>,
        reply_to: mpsc::Sender<RunnerResponse>,
    },
    WriteBuffer {
        value: Vec<u8>,
        id: i32,
        align: u32,
        reply_to: mpsc::Sender<RunnerResponse>,
    },
    GetRemainingGas {
        reply_to: mpsc::Sender<RunnerResponse>,
    },
    IsOutOfMemory {
        reply_to: mpsc::Sender<RunnerResponse>,
    },
    SetRemainingGas {
        gas: u64,
    },
    UseGas {
        gas: u64,
    },
    GetAbortData {
        reply_to: mpsc::Sender<RunnerResponse>,
    },
}
