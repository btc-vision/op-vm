use anyhow::Result as AnyResult;
use bytes::Bytes;
use std::sync::{mpsc, Arc};
use std::thread;
use wasmer::Value;
use wasmer_types::SerializeError;

use crate::domain::runner::runner_response::{RunnerCommand, RunnerResponse};
use crate::domain::runner::{AbortData, ContractRunner, ExtendedMemoryAccessError, WasmerRunner};

// A cloneable handle that sends commands to the background thread.
#[derive(Clone)]
pub struct ThreadedWasmerRunner {
    cmd_tx: Arc<mpsc::Sender<RunnerCommand>>,
}

impl ThreadedWasmerRunner {
    /// Spawns a dedicated worker thread, which holds the real `WasmerRunner`.
    pub fn new(runner: WasmerRunner) -> Self {
        let (sender, receiver) = mpsc::channel::<RunnerCommand>();

        // Move the `runner` into a background thread, process commands in a loop:
        thread::spawn(move || {
            let mut wasmer = runner; // Own the runner here

            while let Ok(cmd) = receiver.recv() {
                match cmd {
                    RunnerCommand::Serialize { reply_to } => {
                        let result = wasmer.serialize();
                        let _ = reply_to.send(RunnerResponse::SerializeResult(result));
                    }
                    RunnerCommand::Call {
                        function,
                        params,
                        reply_to,
                    } => {
                        let result = wasmer.call(&function, &params);
                        let _ = reply_to.send(RunnerResponse::CallResult(result));
                    }
                    RunnerCommand::ReadMemory {
                        offset,
                        length,
                        reply_to,
                    } => {
                        let result = wasmer.read_memory(offset, length);
                        let _ = reply_to.send(RunnerResponse::ReadMemoryResult(result));
                    }
                    RunnerCommand::WriteMemory {
                        offset,
                        data,
                        reply_to,
                    } => {
                        let result = wasmer.write_memory(offset, &data);
                        let _ = reply_to.send(RunnerResponse::WriteMemoryResult(result));
                    }
                    RunnerCommand::WriteBuffer {
                        value,
                        id,
                        align,
                        reply_to,
                    } => {
                        let result = wasmer.write_buffer(&value, id, align);
                        let _ = reply_to.send(RunnerResponse::WriteBufferResult(result));
                    }
                    RunnerCommand::GetRemainingGas { reply_to } => {
                        let gas = wasmer.get_remaining_gas();
                        let _ = reply_to.send(RunnerResponse::RemainingGas(gas));
                    }
                    RunnerCommand::IsOutOfMemory { reply_to } => {
                        let res = wasmer.is_out_of_memory();
                        let _ = reply_to.send(RunnerResponse::OutOfMemory(res));
                    }
                    RunnerCommand::SetRemainingGas { gas } => {
                        wasmer.set_remaining_gas(gas);
                        // no explicit reply needed
                    }
                    RunnerCommand::UseGas { gas } => {
                        wasmer.use_gas(gas);
                        // no explicit reply
                    }
                    RunnerCommand::GetAbortData { reply_to } => {
                        let data = wasmer.get_abort_data();
                        let _ = reply_to.send(RunnerResponse::AbortData(data));
                    }
                }
            }
        });

        // Return a handle that can send commands to the above thread
        ThreadedWasmerRunner {
            cmd_tx: Arc::new(sender),
        }
    }

    /// Serialize the module, blocking until the worker responds.
    pub fn serialize(&self) -> Result<Bytes, SerializeError> {
        let (tx, rx) = mpsc::channel();
        let cmd = RunnerCommand::Serialize { reply_to: tx };
        self.cmd_tx
            .send(cmd)
            .expect("failed to send Serialize command");
        match rx.recv().expect("worker thread hung up") {
            RunnerResponse::SerializeResult(r) => r,
            _ => unreachable!("received unexpected response variant"),
        }
    }

    /// Call an exported function.
    pub fn call(&self, function: &str, params: &[Value]) -> AnyResult<Box<[Value]>> {
        let (tx, rx) = mpsc::channel();
        let cmd = RunnerCommand::Call {
            function: function.to_string(),
            params: params.to_vec(),
            reply_to: tx,
        };
        self.cmd_tx.send(cmd).expect("failed to send Call command");
        match rx.recv().expect("worker thread hung up") {
            RunnerResponse::CallResult(r) => r,
            _ => unreachable!(),
        }
    }

    pub fn read_memory(
        &self,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, ExtendedMemoryAccessError> {
        let (tx, rx) = mpsc::channel();
        let cmd = RunnerCommand::ReadMemory {
            offset,
            length,
            reply_to: tx,
        };
        self.cmd_tx
            .send(cmd)
            .expect("failed to send ReadMemory command");
        match rx.recv().expect("worker thread hung up") {
            RunnerResponse::ReadMemoryResult(r) => r,
            _ => unreachable!(),
        }
    }

    pub fn write_memory(&self, offset: u64, data: &[u8]) -> Result<(), ExtendedMemoryAccessError> {
        let (tx, rx) = mpsc::channel();
        let cmd = RunnerCommand::WriteMemory {
            offset,
            data: data.to_vec(),
            reply_to: tx,
        };
        self.cmd_tx
            .send(cmd)
            .expect("failed to send WriteMemory command");
        match rx.recv().expect("worker thread hung up") {
            RunnerResponse::WriteMemoryResult(r) => r,
            _ => unreachable!(),
        }
    }

    pub fn write_buffer(&self, value: &[u8], id: i32, align: u32) -> Result<i64, napi::Error> {
        let (tx, rx) = mpsc::channel();
        let cmd = RunnerCommand::WriteBuffer {
            value: value.to_vec(),
            id,
            align,
            reply_to: tx,
        };
        self.cmd_tx
            .send(cmd)
            .expect("failed to send WriteBuffer command");
        match rx.recv().expect("worker thread hung up") {
            RunnerResponse::WriteBufferResult(r) => r,
            _ => unreachable!(),
        }
    }

    pub fn get_remaining_gas(&self) -> u64 {
        let (tx, rx) = mpsc::channel();
        let cmd = RunnerCommand::GetRemainingGas { reply_to: tx };
        self.cmd_tx
            .send(cmd)
            .expect("failed to send GetRemainingGas");
        match rx.recv().expect("worker thread hung up") {
            RunnerResponse::RemainingGas(g) => g,
            _ => unreachable!(),
        }
    }

    pub fn is_out_of_memory(&self) -> Result<bool, ExtendedMemoryAccessError> {
        let (tx, rx) = mpsc::channel();
        let cmd = RunnerCommand::IsOutOfMemory { reply_to: tx };
        self.cmd_tx.send(cmd).expect("failed to send IsOutOfMemory");
        match rx.recv().expect("worker thread hung up") {
            RunnerResponse::OutOfMemory(r) => r,
            _ => unreachable!(),
        }
    }

    pub fn set_remaining_gas(&self, gas: u64) {
        let cmd = RunnerCommand::SetRemainingGas { gas };
        self.cmd_tx
            .send(cmd)
            .expect("failed to send SetRemainingGas");
    }

    pub fn use_gas(&self, gas: u64) {
        let cmd = RunnerCommand::UseGas { gas };
        self.cmd_tx.send(cmd).expect("failed to send UseGas");
    }

    pub fn get_abort_data(&self) -> Option<AbortData> {
        let (tx, rx) = mpsc::channel();
        let cmd = RunnerCommand::GetAbortData { reply_to: tx };
        self.cmd_tx.send(cmd).expect("failed to send GetAbortData");
        match rx.recv().expect("worker thread hung up") {
            RunnerResponse::AbortData(d) => d,
            _ => unreachable!(),
        }
    }
}
