use crate::domain::tcp::tcp_socket::SocketConnection;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use wasmer::RuntimeError;

#[derive(Debug)]
pub struct SocketPool {
    address: SocketAddr,
    max_pool_size: usize,
    connections: Arc<Mutex<Vec<Arc<Mutex<SocketConnection>>>>>,
}

impl SocketPool {
    /// Creates a pool that will connect to `127.0.0.1:port`.
    pub fn new(port: u16, max_pool_size: usize) -> std::io::Result<Self> {
        let addr = format!("127.0.0.1:{}", port)
            .to_socket_addrs()?
            .next()
            .expect("invalid address");

        Ok(Self {
            address: addr,
            max_pool_size,
            connections: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Retrieve or create a connection from the pool.
    pub fn get_connection(&self) -> Result<Arc<Mutex<SocketConnection>>, RuntimeError> {
        let mut pool = self
            .connections
            .lock()
            .map_err(|e| RuntimeError::new(format!("Could not lock pool: {:?}", e)))?;

        // If there's an available connection, reuse it:
        if let Some(conn) = pool.pop() {
            return Ok(conn);
        }

        // Otherwise, if not at capacity, create a new one:
        //if pool.len() < self.max_pool_size {
        match SocketConnection::create(self.address) {
            Ok(conn) => Ok(Arc::new(Mutex::new(conn))),
            Err(e) => Err(RuntimeError::new(format!("Could not connect: {:?}", e))),
        }
        //}

        //Err(RuntimeError::new("Maximum pool size reached"))
    }

    /// Return the connection to the pool for reuse.
    pub fn return_connection(
        &self,
        conn: Arc<Mutex<SocketConnection>>,
    ) -> Result<(), RuntimeError> {
        let mut pool = self
            .connections
            .lock()
            .map_err(|e| RuntimeError::new(format!("Could not lock pool: {:?}", e)))?;

        if pool.len() >= self.max_pool_size {
            let c = conn
                .lock()
                .map_err(|e| RuntimeError::new(format!("Could not lock connection: {:?}", e)))?;

            c.close()?;
        }

        pool.push(conn);
        Ok(())
    }
}
