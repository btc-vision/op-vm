use crate::domain::runner::bitcoin_network::BitcoinNetwork;
use crate::domain::runner::{AbortData, InstanceWrapper};
use crate::domain::tcp::SocketConnection;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct CustomEnv {
    pub instance: Option<InstanceWrapper>,
    pub network: BitcoinNetwork,
    pub abort_data: Option<AbortData>,
    //pub runtime: Arc<Runtime>,
    pub socket: Arc<Mutex<SocketConnection>>,
    pub refunded_pointers: Mutex<HashMap<Vec<u8>, bool>>,
}

impl CustomEnv {
    pub fn new(
        network: BitcoinNetwork,
        //runtime: Arc<Runtime>,
        socket: Arc<Mutex<SocketConnection>>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            instance: None,
            network,
            abort_data: None,
            socket,
            //runtime,
            refunded_pointers: Mutex::new(HashMap::new()),
        })
    }
}
