use crate::interfaces::ExternalFunction;
use std::sync::Arc;

macro_rules! dummy_ext_fn {
    ($name:ident, $ret:ty) => {
        #[derive(Clone)]
        pub struct $name(String);
        impl $name {
            pub fn new() -> Self {
                Self(stringify!($name).into())
            }
        }
        impl ExternalFunction<$ret> for $name {
            fn name(&self) -> String {
                self.0.clone()
            }
            fn handle(&self) -> Arc<neon::prelude::Root<neon::types::JsFunction>> {
                unreachable!("JS handle not needed in unit tests")
            }
            fn channel(&self) -> neon::prelude::Channel {
                unreachable!("JS channel not needed in unit tests")
            }
        }
    };
}

dummy_ext_fn!(StorageLoadDummy, Vec<u8>);
dummy_ext_fn!(StorageStoreDummy, ());
dummy_ext_fn!(CallOtherContractDummy, Vec<u8>);
dummy_ext_fn!(DeployFromAddressDummy, u64);
dummy_ext_fn!(UpdateFromAddressDummy, u64);
dummy_ext_fn!(ConsoleLogDummy, ());
dummy_ext_fn!(EmitDummy, ());
dummy_ext_fn!(InputsDummy, Vec<u8>);
dummy_ext_fn!(OutputsDummy, Vec<u8>);
dummy_ext_fn!(AccountTypeDummy, u32);
dummy_ext_fn!(BlockHashDummy, [u8; 32]);

// TODO: Fix this so we can make unit tests.
/*#[derive(Clone, Default, Debug)]
pub struct TransientStorageDummy {
    inner: HashMap<Vec<u8>, Vec<u8>>,
}

impl TransientStorageDummy {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn load(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.inner.get(key).cloned()
    }
    pub fn store(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.inner.insert(key, value);
    }
}

pub fn dummy_custom_env() -> CustomEnv {
    CustomEnv {
        instance: None,
        network: BitcoinNetwork::Mainnet,
        exit_data: ExitData::default(),

        storage_load_external: StorageLoadDummy::new(),
        storage_store_external: StorageStoreDummy::new(),
        call_other_contract_external: CallOtherContractDummy::new(),
        deploy_from_address_external: DeployFromAddressDummy::new(),
        console_log_external: ConsoleLogDummy::new(),
        emit_external: EmitDummy::new(),
        inputs_external: InputsDummy::new(),
        outputs_external: OutputsDummy::new(),
        account_type_external: AccountTypeDummy::new(),
        block_hash_external: BlockHashDummy::new(),

        runtime: Arc::new(Runtime::new().expect("Failed to create Tokio Runtime")),
        calldata: Calldata::default(),
        environment_variables: Some(EnvironmentVariables::default()),
        last_call_result: CallResult::default(),
        is_running_start_function: false,
        transient_storage: TransientStorageDummy::new(),
        max_pages: 32,

        return_proofs: false,
        proofs: Vec::new(),
    }
}*/
