use std::io::{ErrorKind, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wasmer::RuntimeError;

#[derive(Debug)]
pub struct SocketConnection {
    address: SocketAddr,
    stream: Option<Arc<Mutex<TcpStream>>>,
    id: u64,
}

enum Opcode {
    StorageLoad = 0,
    StorageStore = 1,
    CallOtherContract = 2,
    EmitEventExternal = 3,
    DeployFromAddress = 4,
    InputsExternal = 5,
    OutputsExternal = 6,
    ConsoleLogExternal = 7,
}

impl SocketConnection {
    /// Create a SocketConnection struct but do NOT connect yet.
    pub fn create(address: SocketAddr) -> std::io::Result<Self> {
        Ok(Self {
            address,
            stream: None,
            id: 0,
        })
    }

    /// Connect if we don't already have a connection
    pub fn connect_if_not_connected(&mut self) -> Result<(), RuntimeError> {
        if self.stream.is_none() {
            let stream = TcpStream::connect(self.address)
                .map_err(|e| RuntimeError::new(format!("Connection failed: {:?}", e)))?;

            Self::configure_stream(&stream)
                .map_err(|e| RuntimeError::new(format!("Stream config failed: {:?}", e)))?;

            self.stream = Some(Arc::new(Mutex::new(stream)));
        }
        Ok(())
    }

    /// Configure stream settings as desired.
    fn configure_stream(stream: &TcpStream) -> std::io::Result<()> {
        stream.set_read_timeout(Option::from(Duration::from_secs(10)))?; // None
        stream.set_write_timeout(Option::from(Duration::from_secs(10)))?; // None
        stream.set_nodelay(true)?;
        Ok(())
    }

    /// Attempt to reconnect an existing connection in-place.
    fn attempt_reconnect(&self) -> Result<(), RuntimeError> {
        let new_stream = TcpStream::connect(self.address)
            .map_err(|e| RuntimeError::new(format!("Reconnection failed: {:?}", e)))?;
        Self::configure_stream(&new_stream)
            .map_err(|e| RuntimeError::new(format!("Stream config failed: {:?}", e)))?;

        // Replace the old TcpStream with the new one
        if let Some(stream_arc) = &self.stream {
            let mut guard = stream_arc
                .lock()
                .map_err(|_| RuntimeError::new("Lock poisoned while reconnecting"))?;
            *guard = new_stream;
        }
        Ok(())
    }

    /// Close the socket.
    pub fn close(&self) -> Result<(), RuntimeError> {
        // If we have no stream, there's nothing to close.
        let Some(stream_arc) = &self.stream else {
            return Ok(());
        };

        match stream_arc.lock() {
            Ok(stream_guard) => {
                stream_guard.shutdown(Shutdown::Both).map_err(|e| {
                    RuntimeError::new(format!("Error shutting down socket: {:?}", e))
                })?;
                Ok(())
            }
            Err(_) => Err(RuntimeError::new("Lock poisoned while closing socket")),
        }
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    /// Catch panics and convert them to `RuntimeError`.
    fn catch_unwind_as_runtime_error<T, F>(f: F) -> Result<T, RuntimeError>
    where
        F: FnOnce() -> Result<T, RuntimeError> + std::panic::UnwindSafe,
    {
        catch_unwind(AssertUnwindSafe(f)).unwrap_or_else(|_| {
            Err(RuntimeError::new(
                "A panic occurred inside socket operation",
            ))
        })
    }

    /// Write data to the stream with auto-reconnect on certain errors.
    fn safe_write_all(&self, data: &[u8]) -> Result<(), RuntimeError> {
        let Some(stream_arc) = &self.stream else {
            return Err(RuntimeError::new("No TCP stream available for writing"));
        };

        let mut guard = stream_arc
            .lock()
            .map_err(|_| RuntimeError::new("Lock poisoned"))?;
        if let Err(e) = guard.write_all(data) {
            if matches!(
                e.kind(),
                ErrorKind::BrokenPipe
                    | ErrorKind::ConnectionReset
                    | ErrorKind::NotConnected
                    | ErrorKind::ConnectionAborted
            ) {
                drop(guard);
                self.attempt_reconnect()?;
                // Try writing again
                let Some(stream_arc2) = &self.stream else {
                    return Err(RuntimeError::new("No TCP stream after reconnect"));
                };
                let mut guard2 = stream_arc2
                    .lock()
                    .map_err(|_| RuntimeError::new("Lock poisoned after reconnect"))?;
                guard2.write_all(data).map_err(|e2| {
                    RuntimeError::new(format!("Write error after reconnect: {:?}", e2))
                })?;
            } else {
                // Some other error
                return Err(RuntimeError::new(format!("Write error: {:?}", e)));
            }
        }
        Ok(())
    }

    /// Read data from the stream with auto-reconnect on certain errors.
    fn safe_read(&self, buf: &mut [u8]) -> Result<usize, RuntimeError> {
        let Some(stream_arc) = &self.stream else {
            return Err(RuntimeError::new("No TCP stream available for reading"));
        };

        let mut guard = stream_arc
            .lock()
            .map_err(|_| RuntimeError::new("Lock poisoned"))?;

        match guard.read(buf) {
            Ok(n) => Ok(n),
            Err(e) => {
                if matches!(
                    e.kind(),
                    ErrorKind::BrokenPipe
                        | ErrorKind::ConnectionReset
                        | ErrorKind::NotConnected
                        | ErrorKind::ConnectionAborted
                ) {
                    drop(guard);
                    self.attempt_reconnect()?;
                    let Some(stream_arc2) = &self.stream else {
                        return Err(RuntimeError::new("No TCP stream after reconnect"));
                    };

                    let mut guard2 = stream_arc2
                        .lock()
                        .map_err(|_| RuntimeError::new("Lock poisoned after reconnect"))?;
                    guard2.read(buf).map_err(|e2| {
                        RuntimeError::new(format!("Read error after reconnect: {:?}", e2))
                    })
                } else {
                    Err(RuntimeError::new(format!("Read error: {:?}", e)))
                }
            }
        }
    }

    pub fn load(&mut self, data: &Vec<u8>) -> Result<Vec<u8>, RuntimeError> {
        // Connect if not already
        self.connect_if_not_connected()?;

        println!("--  SocketConnection::load() data: {:?}", data);

        let header_buffer = self.get_header(Opcode::StorageLoad, data.len() as u32);
        let final_buffer = [header_buffer.to_vec(), data.to_vec()].concat();

        self.safe_write_all(&final_buffer)?;

        let mut buf = [0u8; 32];
        let n = self.safe_read(&mut buf)?;
        Ok(buf[..n].to_vec())
    }

    pub fn store(&mut self, data: &Vec<u8>) -> Result<Vec<u8>, RuntimeError> {
        self.connect_if_not_connected()?;

        let header_buffer = self.get_header(Opcode::StorageStore, data.len() as u32);
        let final_buffer = [header_buffer.to_vec(), data.to_vec()].concat();

        self.safe_write_all(&final_buffer)?;

        let mut buf = [0u8; 32];
        let n = self.safe_read(&mut buf)?;
        Ok(buf[..n].to_vec())
    }

    pub fn call_other_contract(&mut self, data: &Vec<u8>) -> Result<Vec<u8>, RuntimeError> {
        self.connect_if_not_connected()?;

        let header_buffer = self.get_header(Opcode::CallOtherContract, data.len() as u32);
        let final_buffer = [header_buffer.to_vec(), data.to_vec()].concat();

        self.safe_write_all(&final_buffer)?;

        let mut buf = [0u8; 1024 * 1024]; // limit to 1MB
        let n = self.safe_read(&mut buf)?;

        let resp = &buf[..n];
        // Avoid panicking if stdout fails
        let _ = std::io::Write::write_all(
            &mut std::io::stdout(),
            format!("Response: {:?}\n", resp).as_bytes(),
        );
        Ok(resp.to_vec())
    }

    pub fn inputs_external(&mut self) -> Result<Vec<u8>, RuntimeError> {
        self.connect_if_not_connected()?;

        let header_buffer = self.get_header(Opcode::InputsExternal, 0);
        self.safe_write_all(&header_buffer)?;

        let mut buf = [0u8; 512 * 1024];
        let n = self.safe_read(&mut buf)?;
        Ok(buf[..n].to_vec())
    }

    pub fn outputs_external(&mut self) -> Result<Vec<u8>, RuntimeError> {
        self.connect_if_not_connected()?;

        let header_buffer = self.get_header(Opcode::OutputsExternal, 0);
        self.safe_write_all(&header_buffer)?;

        let mut buf = [0u8; 512 * 1024];
        let n = self.safe_read(&mut buf)?;
        Ok(buf[..n].to_vec())
    }

    pub fn deploy_from_address(&mut self, data: &Vec<u8>) -> Result<Vec<u8>, RuntimeError> {
        self.connect_if_not_connected()?;

        let header_buffer = self.get_header(Opcode::DeployFromAddress, data.len() as u32);
        let final_buffer = [header_buffer.to_vec(), data.to_vec()].concat();

        self.safe_write_all(&final_buffer)?;

        let mut buf = [0u8; 32];
        let n = self.safe_read(&mut buf)?;
        Ok(buf[..n].to_vec())
    }

    pub fn console_log_external(&mut self, data: &Vec<u8>) -> Result<(), RuntimeError> {
        self.connect_if_not_connected()?;

        let header_buffer = self.get_header(Opcode::ConsoleLogExternal, data.len() as u32);
        let final_buffer = [header_buffer.to_vec(), data.to_vec()].concat();

        self.safe_write_all(&final_buffer)?;
        Ok(())
    }

    pub fn emit_event_external(&mut self, data: &Vec<u8>) -> Result<(), RuntimeError> {
        self.connect_if_not_connected()?;

        let header_buffer = self.get_header(Opcode::EmitEventExternal, data.len() as u32);
        let final_buffer = [header_buffer.to_vec(), data.to_vec()].concat();

        self.safe_write_all(&final_buffer)?;
        Ok(())
    }

    fn get_header(&self, opcode: Opcode, length: u32) -> [u8; 13] {
        let mut header_buffer = [0u8; 13];
        header_buffer[0] = opcode as u8;
        header_buffer[1..5].copy_from_slice(&length.to_be_bytes());
        header_buffer[5..13].copy_from_slice(&self.id.to_be_bytes());
        header_buffer
    }
}

impl Drop for SocketConnection {
    fn drop(&mut self) {
        // Guard against panics in `drop`.
        let _ = catch_unwind(AssertUnwindSafe(|| {
            println!("Dropping connection");
            let _ = self.close();
        }));
    }
}
