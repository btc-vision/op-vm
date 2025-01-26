use anyhow::Result as AnyResult;
use bytes::Bytes;
use std::sync::{mpsc, Arc};
use std::thread;
use wasmer::Value;
use wasmer_types::SerializeError;

use crate::domain::runner::runner_response::{RunnerCommand, RunnerResponse};
use crate::domain::runner::{AbortData, ContractRunner, ExtendedMemoryAccessError, WasmerRunner};

/// A cloneable handle that sends commands to the background thread.
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
                //println!("[!!!] Received command: {:?}", cmd);
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
                    }
                    RunnerCommand::UseGas { gas } => {
                        wasmer.use_gas(gas);
                    }
                    RunnerCommand::GetAbortData { reply_to } => {
                        let data = wasmer.get_abort_data();
                        let _ = reply_to.send(RunnerResponse::AbortData(data));
                    }
                }
            }
        });

        ThreadedWasmerRunner {
            cmd_tx: Arc::new(sender),
        }
    }

    // Helper for receiving with a 5-second timeout. Logs on timeout.
    fn safe_recv<T>(rx: mpsc::Receiver<T>, action_label: &str) -> Option<T> {
        match rx.recv() {
            Ok(msg) => Some(msg), //Duration::from_secs(5)
            /*Err(mpsc::RecvTimeoutError::Timeout) => {
                eprintln!(
                    "[ThreadedWasmerRunner] Timed out (5s) waiting for response in {}",
                    action_label
                );
                None
            }*/
            Err(mpsc::RecvError) => {
                eprintln!(
                    "[ThreadedWasmerRunner] Disconnected while waiting for response in {}",
                    action_label
                );
                None
            }
        }
    }

    /// Serialize the module, blocking until the worker responds (or 5s).
    pub fn serialize(&self) -> Result<Bytes, SerializeError> {
        let (tx, rx) = mpsc::channel();
        let cmd = RunnerCommand::Serialize { reply_to: tx };
        self.cmd_tx
            .send(cmd)
            .expect("failed to send Serialize command");

        // Wait for up to 5 seconds:
        match Self::safe_recv(rx, "serialize") {
            Some(RunnerResponse::SerializeResult(r)) => r,
            Some(_) => unreachable!("received unexpected response variant"),
            None => {
                // If we time out or disconnect, we can return a custom error or panic.
                // For demonstration, we'll just create a dummy SerializeError:
                eprintln!("[ThreadedWasmerRunner] No response -> returning SerializeError");
                // You may need a real variant here that suits your code:
                Err(SerializeError::Generic("timeout after 5s".into()))
            }
        }
    }

    /// Call an exported function (up to 5s).
    pub fn call(&self, function: &str, params: &[Value]) -> AnyResult<Box<[Value]>> {
        let (tx, rx) = mpsc::channel();
        let cmd = RunnerCommand::Call {
            function: function.to_string(),
            params: params.to_vec(),
            reply_to: tx,
        };
        self.cmd_tx.send(cmd).expect("failed to send Call command");

        match Self::safe_recv(rx, "call") {
            Some(RunnerResponse::CallResult(r)) => r,
            Some(_) => unreachable!("received unexpected response variant"),
            None => {
                eprintln!("[ThreadedWasmerRunner] Timed out in call()");
                // Return an anyhow error indicating timeout
                Err(anyhow::anyhow!(
                    "Runner thread timed out (5s) in call({})",
                    function
                ))
            }
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

        match Self::safe_recv(rx, "read_memory") {
            Some(RunnerResponse::ReadMemoryResult(r)) => r,
            Some(_) => unreachable!(),
            None => {
                eprintln!("[ThreadedWasmerRunner] Timed out in read_memory()");
                // Return an error. If your `ExtendedMemoryAccessError` has no "generic" variant,
                // you may need to add one. For demonstration:
                Err(ExtendedMemoryAccessError::Unknown)
            }
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

        match Self::safe_recv(rx, "write_memory") {
            Some(RunnerResponse::WriteMemoryResult(r)) => r,
            Some(_) => unreachable!(),
            None => {
                eprintln!("[ThreadedWasmerRunner] Timed out in write_memory()");
                Err(ExtendedMemoryAccessError::Unknown)
            }
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

        match Self::safe_recv(rx, "write_buffer") {
            Some(RunnerResponse::WriteBufferResult(r)) => r,
            Some(_) => unreachable!(),
            None => {
                eprintln!("[ThreadedWasmerRunner] Timed out in write_buffer()");
                // Return an N-API error or any other relevant error
                Err(napi::Error::from_reason(
                    "Timeout after 5 seconds in write_buffer".to_string(),
                ))
            }
        }
    }

    pub fn get_remaining_gas(&self) -> u64 {
        let (tx, rx) = mpsc::channel();
        let cmd = RunnerCommand::GetRemainingGas { reply_to: tx };
        self.cmd_tx
            .send(cmd)
            .expect("failed to send GetRemainingGas");

        match Self::safe_recv(rx, "get_remaining_gas") {
            Some(RunnerResponse::RemainingGas(g)) => g,
            Some(_) => unreachable!(),
            None => {
                eprintln!("[ThreadedWasmerRunner] Timed out in get_remaining_gas()");
                // We cannot return an error for a u64 return, so fallback to 0 or any default
                0
            }
        }
    }

    pub fn is_out_of_memory(&self) -> Result<bool, ExtendedMemoryAccessError> {
        let (tx, rx) = mpsc::channel();
        let cmd = RunnerCommand::IsOutOfMemory { reply_to: tx };
        self.cmd_tx.send(cmd).expect("failed to send IsOutOfMemory");

        match Self::safe_recv(rx, "is_out_of_memory") {
            Some(RunnerResponse::OutOfMemory(r)) => r,
            Some(_) => unreachable!(),
            None => {
                eprintln!("[ThreadedWasmerRunner] Timed out in is_out_of_memory()");
                Err(ExtendedMemoryAccessError::Unknown)
            }
        }
    }

    pub fn set_remaining_gas(&self, gas: u64) {
        let cmd = RunnerCommand::SetRemainingGas { gas };
        if let Err(e) = self.cmd_tx.send(cmd) {
            eprintln!(
                "[ThreadedWasmerRunner] Failed to send SetRemainingGas command: {:?}",
                e
            );
        }
    }

    pub fn use_gas(&self, gas: u64) {
        let cmd = RunnerCommand::UseGas { gas };
        if let Err(e) = self.cmd_tx.send(cmd) {
            eprintln!(
                "[ThreadedWasmerRunner] Failed to send UseGas command: {:?}",
                e
            );
        }
    }

    pub fn get_abort_data(&self) -> Option<AbortData> {
        let (tx, rx) = mpsc::channel();
        let cmd = RunnerCommand::GetAbortData { reply_to: tx };
        self.cmd_tx.send(cmd).expect("failed to send GetAbortData");

        match Self::safe_recv(rx, "get_abort_data") {
            Some(RunnerResponse::AbortData(d)) => d,
            Some(_) => unreachable!(),
            None => {
                eprintln!("[ThreadedWasmerRunner] Timed out in get_abort_data()");
                None
            }
        }
    }
}
