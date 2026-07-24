use bytes::Bytes;
use chrono::Local;
use std::sync::Arc;
use wasmer::sys::{BaseTunables, EngineBuilder};
use wasmer::{imports, Function, FunctionEnv, Instance, Module, RuntimeError, Store, Value};
use wasmer_compiler::{CompilerConfig, Engine};
use wasmer_compiler_singlepass::Singlepass;
use wasmer_types::target::Target;
use wasmer_types::{Features, SerializeError};

use crate::domain::runner::constants::STACK_SIZE;
#[allow(unused_imports)]
use crate::domain::runner::{
    CallOtherContractImport, Calldata, ConsensusFlags, ConsoleLogImport, ContractRunner, CustomEnv,
    DeployFromAddressImport, EmitImport, EnvironmentVariables, ExitData, ExitImport, ExitResult,
    ExtendedMemoryAccessError, GetAccountTypeImport, GetBlockHashImport, GetCallResultImport,
    GetCalldataImport, GetEnvironmentVariablesImport, GetInputsImport, GetInputsSizeImport,
    GetOutputsImport, GetOutputsSizeImport, InstanceWrapper, Ripemd160Import, Sha256Import,
    StorageLoadImport, StorageStoreImport, TransientLoadImport, TransientStoreImport,
    ValidateBitcoinAddressImport, VerifySignatureImport, MAX_GAS_WASM_INIT,
};

use crate::domain::vm::{
    get_gas_cost, log_time_diff, LimitingTunables, Metering, RejectFPMiddleware,
};

use crate::domain::runner::MAX_TABLE_ELEMENTS;
use crate::domain::runner::{
    MLDSALoadImport, UpdateFromAddressImport, MAX_MEMORY_COPY_SIZE, MAX_MEMORY_SIZE,
};

const CONTRACT_ENTRYPOINT_FUNCTION_NAME: &'static str = "execute";
const CONTRACT_ON_DEPLOY_FUNCTION_NAME: &'static str = "onDeploy";
const CONTRACT_ON_UPDATE_FUNCTION_NAME: &'static str = "onUpdate";

pub struct WasmerRunner {
    module: Module,
    store: Store,
    instance: InstanceWrapper,
    env: FunctionEnv<CustomEnv>,
}

impl WasmerRunner {
    pub fn from_bytecode(
        bytecode: &[u8],
        used_gas: u64,
        max_gas: u64,
        max_pages: u32,
        consensus_flags: ConsensusFlags,
        custom_env: CustomEnv,
        is_debug_mode: bool,
    ) -> anyhow::Result<Self> {
        let time = Local::now();

        if consensus_flags.contains(ConsensusFlags::STRICT_MEMORY_METERING) {
            Self::validate_bytecode_for_strict_memory_metering(bytecode, max_pages)?;
        }

        let store = Self::create_engine_with_consensus_flags(max_pages, consensus_flags)?;
        let module = Module::from_binary(&store, &bytecode)?;
        if consensus_flags.contains(ConsensusFlags::STRICT_MEMORY_METERING) {
            Self::validate_module_for_strict_memory_metering(&module, max_pages)?;
        }
        let instance =
            Self::create_instance(used_gas, max_gas, custom_env, store, module, is_debug_mode)?;

        log_time_diff(&time, "WasmerRunner::from_bytecode");

        Ok(instance)
    }

    pub fn create_engine(max_pages: u32) -> anyhow::Result<Store> {
        Self::create_engine_with_consensus_flags(max_pages, ConsensusFlags::NONE)
    }

    pub fn create_engine_with_consensus_flags(
        max_pages: u32,
        consensus_flags: ConsensusFlags,
    ) -> anyhow::Result<Store> {
        let strict_memory_metering =
            consensus_flags.contains(ConsensusFlags::STRICT_MEMORY_METERING);
        let meter = Metering::new_with_strict_memory_metering(
            MAX_GAS_WASM_INIT,
            get_gas_cost,
            MAX_MEMORY_COPY_SIZE,
            strict_memory_metering,
        );
        let metering = Arc::new(meter);

        let mut compiler = Singlepass::default();
        compiler.canonicalize_nans(true);

        compiler.enable_verifier();

        compiler.push_middleware(RejectFPMiddleware::new());
        compiler.push_middleware(metering);

        let engine = EngineBuilder::new(compiler)
            .set_features(Option::from(Self::get_features()))
            .engine();

        let store = Store::new(Self::create_tunable(
            engine,
            max_pages,
            strict_memory_metering,
        ));
        Ok(store)
    }

    fn get_features() -> Features {
        let mut features = Features::default();

        // Not deterministic
        features.relaxed_simd = false; // https://github.com/WebAssembly/relaxed-simd/blob/main/proposals/relaxed-simd/Overview.md

        // DoS: MUST BE DISABLED FEATURES
        features.tail_call = false; // Turns infinite self-tail recursion into a single metered opcode; impossible to bound with stack-depth limits.

        features.threads = false;

        // Bad
        features.memory64 = false; // TODO: Enabling this break metering for gas and memory. We need two meters. Larger binaries and ~2–10 % slower memory-heavy loops.

        // Length-aware gas was implemented.
        #[cfg(feature = "reference-types")]
        {
            features.reference_types = true;
        }

        #[cfg(not(feature = "reference-types"))]
        {
            features.reference_types = false;
        }

        features.multi_memory = false; // TODO: Multiple memories break the single-meter assumption. Support must be added.
        features.exceptions = false; // Gas free operation. Not in metering. Can be used to bypass gas limits. Separate opcodes for catch and throw.

        // Could be cool if implemented, maybe useful for loading other contracts in the same module?
        features.module_linking = false; // https://github.com/WebAssembly/module-linking/blob/main/proposals/module-linking/Explainer.md
        features.bulk_memory = true;

        // Ok
        features.simd = true;
        features.multi_value = true;
        features.extended_const = true;

        features
    }

    pub fn serialize(&self) -> anyhow::Result<Bytes, SerializeError> {
        let serialized = self.module.serialize()?;

        Ok(serialized)
    }

    pub unsafe fn from_serialized(
        serialized: Bytes,
        used_gas: u64,
        max_gas: u64,
        max_pages: u32,
        consensus_flags: ConsensusFlags,
        custom_env: CustomEnv,
        is_debug_mode: bool,
    ) -> anyhow::Result<Self> {
        let time = Local::now();
        let strict_memory_metering =
            consensus_flags.contains(ConsensusFlags::STRICT_MEMORY_METERING);

        let engine = EngineBuilder::headless()
            .set_features(Option::from(Self::get_features()))
            .engine();

        let store = Store::new(Self::create_tunable(
            engine,
            max_pages,
            strict_memory_metering,
        ));
        let module = Module::deserialize(&store, serialized)?;
        if strict_memory_metering {
            Self::validate_module_for_strict_memory_metering(&module, max_pages)?;
        }
        let instance =
            Self::create_instance(used_gas, max_gas, custom_env, store, module, is_debug_mode)?;

        log_time_diff(&time, "WasmerRunner::from_serialized");

        Ok(instance)
    }

    fn create_tunable(mut engine: Engine, max_pages: u32, strict_memory_metering: bool) -> Engine {
        let base = BaseTunables::for_target(&Target::default());
        let tunables = LimitingTunables::new(base, max_pages, STACK_SIZE, strict_memory_metering);

        engine.set_tunables(tunables);

        engine
    }

    fn validate_bytecode_for_strict_memory_metering(
        bytecode: &[u8],
        max_pages: u32,
    ) -> anyhow::Result<()> {
        use wasmer::wasmparser::{ElementItems, Imports, Parser, Payload, TypeRef};

        fn validate_memory_type(
            memory: &wasmer::wasmparser::MemoryType,
            max_pages: u32,
        ) -> anyhow::Result<()> {
            if memory.memory64 {
                return Err(anyhow::anyhow!("memory64 is not allowed"));
            }

            if memory.initial > max_pages as u64 {
                return Err(anyhow::anyhow!(
                    "memory minimum exceeds the allowed memory limit"
                ));
            }

            if let Some(maximum) = memory.maximum {
                if maximum > max_pages as u64 {
                    return Err(anyhow::anyhow!(
                        "memory maximum exceeds the allowed memory limit"
                    ));
                }
            }

            Ok(())
        }

        fn validate_table_type(table: &wasmer::wasmparser::TableType) -> anyhow::Result<()> {
            if table.table64 {
                return Err(anyhow::anyhow!("table64 is not allowed"));
            }

            if table.initial > MAX_TABLE_ELEMENTS as u64 {
                return Err(anyhow::anyhow!(
                    "table minimum exceeds the allowed table limit"
                ));
            }

            if let Some(maximum) = table.maximum {
                if maximum > MAX_TABLE_ELEMENTS as u64 {
                    return Err(anyhow::anyhow!(
                        "table maximum exceeds the allowed table limit"
                    ));
                }
            }

            Ok(())
        }

        fn validate_type_ref(ty: TypeRef, max_pages: u32) -> anyhow::Result<()> {
            match ty {
                TypeRef::Memory(memory) => validate_memory_type(&memory, max_pages),
                TypeRef::Table(table) => validate_table_type(&table),
                _ => Ok(()),
            }
        }

        for payload in Parser::new(0).parse_all(bytecode) {
            match payload? {
                Payload::ImportSection(section) => {
                    for import in section {
                        match import? {
                            Imports::Single(_, import) => {
                                validate_type_ref(import.ty, max_pages)?;
                            }
                            Imports::Compact1 { items, .. } => {
                                for item in items {
                                    validate_type_ref(item?.ty, max_pages)?;
                                }
                            }
                            Imports::Compact2 { ty, .. } => {
                                validate_type_ref(ty, max_pages)?;
                            }
                        }
                    }
                }
                Payload::MemorySection(section) => {
                    for memory in section {
                        validate_memory_type(&memory?, max_pages)?;
                    }
                }
                Payload::TableSection(section) => {
                    for table in section {
                        validate_table_type(&table?.ty)?;
                    }
                }
                Payload::DataSection(section) => {
                    let mut total_data_len = 0usize;
                    for data in section {
                        let data = data?;
                        let data_len = data.data.len();
                        if data_len > MAX_MEMORY_COPY_SIZE as usize {
                            return Err(anyhow::anyhow!(
                                "data segment exceeds the allowed memory initialization limit"
                            ));
                        }

                        total_data_len = total_data_len.checked_add(data_len).ok_or_else(|| {
                            anyhow::anyhow!(
                                "data section exceeds the allowed memory initialization limit"
                            )
                        })?;
                        if total_data_len > MAX_MEMORY_SIZE as usize {
                            return Err(anyhow::anyhow!(
                                "data section exceeds the allowed memory initialization limit"
                            ));
                        }
                    }
                }
                Payload::ElementSection(section) => {
                    let mut total_element_count = 0u64;
                    for element in section {
                        let element = element?;
                        let count = u64::from(match element.items {
                            ElementItems::Functions(functions) => functions.count(),
                            ElementItems::Expressions(_, expressions) => expressions.count(),
                        });

                        if count > MAX_TABLE_ELEMENTS as u64 {
                            return Err(anyhow::anyhow!(
                                "element segment exceeds the allowed table initialization limit"
                            ));
                        }

                        total_element_count =
                            total_element_count.checked_add(count).ok_or_else(|| {
                                anyhow::anyhow!(
                                    "element sections exceed the allowed table initialization limit"
                                )
                            })?;
                        if total_element_count > MAX_TABLE_ELEMENTS as u64 {
                            return Err(anyhow::anyhow!(
                                "element sections exceed the allowed table initialization limit"
                            ));
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn validate_module_for_strict_memory_metering(
        module: &Module,
        max_pages: u32,
    ) -> anyhow::Result<()> {
        let info = module.info();

        for (_, memory) in info.memories.iter() {
            if memory.minimum.0 > max_pages {
                return Err(anyhow::anyhow!(
                    "memory minimum exceeds the allowed memory limit"
                ));
            }

            if let Some(maximum) = memory.maximum {
                if maximum.0 > max_pages {
                    return Err(anyhow::anyhow!(
                        "memory maximum exceeds the allowed memory limit"
                    ));
                }
            }
        }

        for (_, table) in info.tables.iter() {
            if table.minimum > MAX_TABLE_ELEMENTS {
                return Err(anyhow::anyhow!(
                    "table minimum exceeds the allowed table limit"
                ));
            }

            if let Some(maximum) = table.maximum {
                if maximum > MAX_TABLE_ELEMENTS {
                    return Err(anyhow::anyhow!(
                        "table maximum exceeds the allowed table limit"
                    ));
                }
            }
        }

        let mut total_table_elements = 0usize;
        for initializer in &info.table_initializers {
            let element_count = initializer.elements.len();
            if element_count > MAX_TABLE_ELEMENTS as usize {
                return Err(anyhow::anyhow!(
                    "table initializer exceeds the allowed table initialization limit"
                ));
            }

            total_table_elements =
                total_table_elements
                    .checked_add(element_count)
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "table initializers exceed the allowed table initialization limit"
                        )
                    })?;
            if total_table_elements > MAX_TABLE_ELEMENTS as usize {
                return Err(anyhow::anyhow!(
                    "table initializers exceed the allowed table initialization limit"
                ));
            }
        }

        for elements in info.passive_elements.values() {
            let element_count = elements.len();
            if element_count > MAX_TABLE_ELEMENTS as usize {
                return Err(anyhow::anyhow!(
                    "passive element segment exceeds the allowed table initialization limit"
                ));
            }

            total_table_elements =
                total_table_elements
                    .checked_add(element_count)
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "table initializers exceed the allowed table initialization limit"
                        )
                    })?;
            if total_table_elements > MAX_TABLE_ELEMENTS as usize {
                return Err(anyhow::anyhow!(
                    "table initializers exceed the allowed table initialization limit"
                ));
            }
        }

        let mut total_passive_data_len = 0usize;
        for data in info.passive_data.values() {
            let data_len = data.len();
            if data_len > MAX_MEMORY_COPY_SIZE as usize {
                return Err(anyhow::anyhow!(
                    "passive data segment exceeds the allowed memory initialization limit"
                ));
            }

            total_passive_data_len =
                total_passive_data_len
                    .checked_add(data_len)
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "passive data segments exceed the allowed memory initialization limit"
                        )
                    })?;
            if total_passive_data_len > MAX_MEMORY_SIZE as usize {
                return Err(anyhow::anyhow!(
                    "passive data segments exceed the allowed memory initialization limit"
                ));
            }
        }

        Ok(())
    }

    fn create_instance(
        used_gas: u64,
        max_gas: u64,
        custom_env: CustomEnv,
        mut store: Store,
        module: Module,
        is_debug_mode: bool,
    ) -> anyhow::Result<Self> {
        let remaining_gas = Self::calculate_remaining_gas(used_gas, max_gas)?;

        let env = FunctionEnv::new(&mut store, custom_env);

        macro_rules! import {
            ($obj:tt) => {
                Function::new_typed_with_env(&mut store, &env, $obj::execute)
            };
        }

        let mut import_object = imports! {
            "env" => {
                "exit" => import!(ExitImport),
                "environment" => import!(GetEnvironmentVariablesImport),
                "calldata" => import!(GetCalldataImport),
                "load" => import!(StorageLoadImport),
                "store" => import!(StorageStoreImport),
                "call" => import!(CallOtherContractImport),
                "callResult" => import!(GetCallResultImport),
                "deployFromAddress" => import!(DeployFromAddressImport),
                "emit" => import!(EmitImport),
                "inputs" => import!(GetInputsImport),
                "inputsSize" => import!(GetInputsSizeImport),
                "outputs" => import!(GetOutputsImport),
                "outputsSize" => import!(GetOutputsSizeImport),
                "sha256" => import!(Sha256Import),
                "ripemd160" => import!(Ripemd160Import),
                "accountType" => import!(GetAccountTypeImport),
                "blockHash" => import!(GetBlockHashImport),
                "validateBitcoinAddress" => import!(ValidateBitcoinAddressImport),
                "verifySignature" => import!(VerifySignatureImport),
                "loadMLDSA" => import!(MLDSALoadImport),
                "updateFromAddress" => import!(UpdateFromAddressImport),
            },
        };

        #[cfg(feature = "transient-storage")]
        {
            import_object.define("env", "tload", import!(TransientLoadImport));
            import_object.define("env", "tstore", import!(TransientStoreImport));
        }

        if is_debug_mode {
            import_object.define("debug", "log", import!(ConsoleLogImport));
        }

        let instance = Instance::new(&mut store, &module, &import_object).map_err(|e| {
            if e.to_string().contains("unreachable") {
                anyhow::anyhow!(
                    "out of gas during initialization (consumed: {})",
                    MAX_GAS_WASM_INIT
                )
            } else {
                anyhow::anyhow!("Failed to instantiate contract: {}", e)
            }
        })?;

        let instance_wrapper = InstanceWrapper::new(instance.clone(), max_gas);
        env.as_mut(&mut store).instance = Some(instance_wrapper.clone());

        let mut imp = Self {
            module,
            store,
            instance: instance_wrapper,
            env,
        };

        let gas_used_by_start_function = imp.get_remaining_gas();
        if gas_used_by_start_function > remaining_gas {
            return Err(anyhow::anyhow!(
                "out of gas (consumed: {})",
                gas_used_by_start_function
            ));
        }

        let remaining_gas = remaining_gas - gas_used_by_start_function;
        imp.set_remaining_gas(remaining_gas);

        let start_function = instance
            .exports
            .get_function("start")
            .map_err(|_| anyhow::anyhow!("OP_NET: start function not found"))?;

        imp.set_is_running_start(true);
        let result_start = start_function.call(&mut imp.store, &[]);
        imp.set_is_running_start(false);

        if let Err(e) = result_start {
            if e.to_string().contains("unreachable") {
                return Err(anyhow::anyhow!(
                    "out of gas during start function (consumed: {})",
                    max_gas
                ));
            }

            let exit_data_clone = imp.env.as_ref(&imp.store).exit_data.clone();
            if !exit_data_clone.is_ok() {
                return Err(anyhow::anyhow!(
                    "Failed to call start function: {}",
                    exit_data_clone
                ));
            }

            return Err(anyhow::anyhow!("Failed to call start function: {}", e));
        }

        Ok(imp)
    }

    fn calculate_remaining_gas(used_gas: u64, max_gas: u64) -> anyhow::Result<u64> {
        if MAX_GAS_WASM_INIT > max_gas {
            return Err(anyhow::anyhow!(
                "too little gas, minimum is {}",
                MAX_GAS_WASM_INIT
            ));
        }

        if used_gas > max_gas {
            return Err(anyhow::anyhow!(
                "too little gas remaining (used: {}, max: {})",
                used_gas,
                max_gas
            ));
        }

        let remaining_gas = max_gas - used_gas;
        if remaining_gas < MAX_GAS_WASM_INIT {
            return Err(anyhow::anyhow!(
                "too little gas remaining, minimum is {}",
                MAX_GAS_WASM_INIT
            ));
        }

        // Always charge the maximum gas for the initialization, we have no way of calculating the
        // actual gas used since this is done in internal wasmer functions and this is used to inject
        // metering. This is a safe operation since we have already checked that the remaining gas
        // is enough.
        Ok(remaining_gas)
    }

    fn set_is_running_start(&mut self, value: bool) {
        let env = self.env.as_mut(&mut self.store);
        env.is_running_start_function = value;
    }

    fn handle_errors(
        &mut self,
        response: Result<Box<[Value]>, RuntimeError>,
        max_gas: u64,
    ) -> anyhow::Result<Box<[Value]>> {
        response.map_err(|e| {
            let gas_used = self.get_remaining_gas();
            if gas_used == 0 {
                anyhow::anyhow!("out of gas (consumed: {})", max_gas)
            } else {
                let out_of_memory = self.is_out_of_memory().unwrap_or(false);
                if out_of_memory {
                    anyhow::anyhow!("out of memory")
                } else {
                    anyhow::anyhow!(e)
                }
            }
        })
    }
}

//#[async_trait::async_trait]
impl ContractRunner for WasmerRunner {
    fn set_environment_variables(&mut self, environment_variables: EnvironmentVariables) {
        let env = self.env.as_mut(&mut self.store);
        env.set_consensus_flags(environment_variables.consensus_flags());
        env.environment_variables = Some(environment_variables);
    }

    fn on_deploy(&mut self, calldata: Vec<u8>, max_gas: u64) -> anyhow::Result<ExitData> {
        let env = self.env.as_mut(&mut self.store);
        env.calldata = Calldata::new(&calldata);

        let export = self
            .instance
            .get_function(CONTRACT_ON_DEPLOY_FUNCTION_NAME)?;

        let params = &[Value::I32(calldata.len() as i32)];
        let response = export.call(&mut self.store, params);

        let response: Result<Box<[Value]>, RuntimeError> = match response {
            Ok(result) => Ok(result),
            Err(error) => match error.downcast::<ExitResult>() {
                Ok(result) => match result {
                    ExitResult::Ok(data) => return Ok(data),
                    ExitResult::Err(e) => Err(e)?,
                },
                Err(e) => Err(e),
            },
        };

        let result = self.handle_errors(response, max_gas)?;

        let status = result
            .get(0)
            .ok_or(RuntimeError::new("Invalid value returned from contract"))?
            .i32()
            .ok_or(RuntimeError::new("Invalid value returned from contract"))?
            as u32;

        let gas_used = self.get_used_gas();
        let env = self.env.as_mut(&mut self.store);

        env.exit_data = ExitData::new(status, gas_used, &[], env.proofs.clone());

        Ok(env.exit_data.clone())
    }

    fn on_update(&mut self, calldata: Vec<u8>, max_gas: u64) -> anyhow::Result<ExitData> {
        let env = self.env.as_mut(&mut self.store);
        env.calldata = Calldata::new(&calldata);

        let export = self
            .instance
            .get_function(CONTRACT_ON_UPDATE_FUNCTION_NAME)?;

        let params = &[Value::I32(calldata.len() as i32)];
        let response = export.call(&mut self.store, params);

        let response: Result<Box<[Value]>, RuntimeError> = match response {
            Ok(result) => Ok(result),
            Err(error) => match error.downcast::<ExitResult>() {
                Ok(result) => match result {
                    ExitResult::Ok(data) => return Ok(data),
                    ExitResult::Err(e) => Err(e)?,
                },
                Err(e) => Err(e),
            },
        };

        let result = self.handle_errors(response, max_gas)?;

        let status = result
            .get(0)
            .ok_or(RuntimeError::new("Invalid value returned from contract"))?
            .i32()
            .ok_or(RuntimeError::new("Invalid value returned from contract"))?
            as u32;

        let gas_used = self.get_used_gas();
        let env = self.env.as_mut(&mut self.store);

        env.exit_data = ExitData::new(status, gas_used, &[], env.proofs.clone());

        Ok(env.exit_data.clone())
    }

    fn execute(&mut self, calldata: Vec<u8>, max_gas: u64) -> anyhow::Result<ExitData> {
        let time = Local::now();
        let env = self.env.as_mut(&mut self.store);
        env.calldata = Calldata::new(&calldata);

        let export = self
            .instance
            .get_function(CONTRACT_ENTRYPOINT_FUNCTION_NAME)?;

        let params = &[Value::I32(calldata.len() as i32)];
        let response = export.call(&mut self.store, params);

        let response: Result<Box<[Value]>, RuntimeError> = match response {
            Ok(result) => Ok(result),
            Err(error) => match error.downcast::<ExitResult>() {
                Ok(result) => match result {
                    ExitResult::Ok(data) => return Ok(data),
                    ExitResult::Err(e) => Err(e)?,
                },
                Err(e) => Err(e),
            },
        };

        let result = self.handle_errors(response, max_gas)?;

        let status = result
            .get(0)
            .ok_or(RuntimeError::new("Invalid value returned from contract"))?
            .i32()
            .ok_or(RuntimeError::new("Invalid value returned from contract"))?
            as u32;

        let gas_used = self.get_used_gas();
        let env = self.env.as_mut(&mut self.store);
        env.exit_data = ExitData::new(status, gas_used, &[], env.proofs.clone());

        log_time_diff(&time, "WasmerRunner::execute");
        Ok(env.exit_data.clone())
    }

    /*fn call_export_by_name(
        &mut self,
        function_name: &str,
        params: &[Value],
        max_gas: u64,
    ) -> anyhow::Result<Box<[Value]>> {
        let export = self.instance.get_function(function_name)?;
        let response = export.call(&mut self.store, params);
        self.handle_errors(response, max_gas)
    }*/

    fn read_memory(&self, offset: u64, length: u64) -> Result<Vec<u8>, ExtendedMemoryAccessError> {
        self.instance.read_memory(&self.store, offset, length)
    }

    fn write_memory(&self, offset: u64, data: &[u8]) -> Result<(), ExtendedMemoryAccessError> {
        self.instance.write_memory(&self.store, offset, data)
    }

    fn get_remaining_gas(&mut self) -> u64 {
        self.instance.get_remaining_gas(&mut self.store)
    }

    fn get_used_gas(&mut self) -> u64 {
        self.instance.get_used_gas(&mut self.store)
    }

    fn is_out_of_memory(&self) -> Result<bool, ExtendedMemoryAccessError> {
        self.instance.is_out_of_memory(&self.store)
    }

    fn set_remaining_gas(&mut self, gas: u64) {
        self.instance.set_remaining_gas(&mut self.store, gas)
    }

    fn use_gas(&mut self, gas: u64) {
        self.instance.use_gas(&mut self.store, gas)
    }

    fn get_exit_data(&self) -> ExitData {
        self.env.as_ref(&self.store).exit_data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::runner::MAX_PAGES;
    use wasmer::{imports, Instance};

    #[test]
    fn strict_validation_rejects_table_minimum_above_limit() {
        let wasm = wat::parse_str(format!(
            r#"
            (module
              (table {} funcref)
            )
            "#,
            MAX_TABLE_ELEMENTS + 1
        ))
        .unwrap();

        let err = WasmerRunner::validate_bytecode_for_strict_memory_metering(&wasm, MAX_PAGES)
            .unwrap_err();

        assert!(
            err.to_string().contains("table minimum exceeds"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn strict_validation_rejects_memory_minimum_above_limit() {
        let wasm = wat::parse_str(format!(
            r#"
            (module
              (memory {} {})
            )
            "#,
            MAX_PAGES + 1,
            MAX_PAGES + 1
        ))
        .unwrap();

        let err = WasmerRunner::validate_bytecode_for_strict_memory_metering(&wasm, MAX_PAGES)
            .unwrap_err();

        assert!(
            err.to_string().contains("memory minimum exceeds"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn strict_validation_rejects_memory_maximum_above_limit() {
        let wasm = wat::parse_str(format!(
            r#"
            (module
              (memory 1 {})
            )
            "#,
            MAX_PAGES + 1
        ))
        .unwrap();

        let err = WasmerRunner::validate_bytecode_for_strict_memory_metering(&wasm, MAX_PAGES)
            .unwrap_err();

        assert!(
            err.to_string().contains("memory maximum exceeds"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn strict_validation_rejects_imported_memory_above_limit() {
        let wasm = wat::parse_str(format!(
            r#"
            (module
              (import "env" "memory" (memory 1 {}))
            )
            "#,
            MAX_PAGES + 1
        ))
        .unwrap();

        let err = WasmerRunner::validate_bytecode_for_strict_memory_metering(&wasm, MAX_PAGES)
            .unwrap_err();

        assert!(
            err.to_string().contains("memory maximum exceeds"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn strict_validation_rejects_table_maximum_above_limit() {
        let wasm = wat::parse_str(format!(
            r#"
            (module
              (table 1 {} funcref)
            )
            "#,
            MAX_TABLE_ELEMENTS + 1
        ))
        .unwrap();

        let err = WasmerRunner::validate_bytecode_for_strict_memory_metering(&wasm, MAX_PAGES)
            .unwrap_err();

        assert!(
            err.to_string().contains("table maximum exceeds"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn strict_validation_rejects_imported_table_above_limit() {
        let wasm = wat::parse_str(format!(
            r#"
            (module
              (import "env" "table" (table 1 {} funcref))
            )
            "#,
            MAX_TABLE_ELEMENTS + 1
        ))
        .unwrap();

        let err = WasmerRunner::validate_bytecode_for_strict_memory_metering(&wasm, MAX_PAGES)
            .unwrap_err();

        assert!(
            err.to_string().contains("table maximum exceeds"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn strict_validation_accepts_memory_and_table_at_exact_limits() {
        let wasm = wat::parse_str(format!(
            r#"
            (module
              (memory 1 {})
              (table {} {} funcref)
            )
            "#,
            MAX_PAGES, MAX_TABLE_ELEMENTS, MAX_TABLE_ELEMENTS
        ))
        .unwrap();

        WasmerRunner::validate_bytecode_for_strict_memory_metering(&wasm, MAX_PAGES).unwrap();
    }

    #[test]
    fn strict_tunables_reject_large_table_instantiation_but_roswell_keeps_legacy_behavior() {
        let wasm = wat::parse_str(format!(
            r#"
            (module
              (table {} funcref)
            )
            "#,
            MAX_TABLE_ELEMENTS + 1
        ))
        .unwrap();

        let mut roswell_store = WasmerRunner::create_engine(MAX_PAGES).unwrap();
        let roswell_module = Module::new(&roswell_store, &wasm).unwrap();
        assert!(Instance::new(&mut roswell_store, &roswell_module, &imports! {}).is_ok());

        let strict_flags = ConsensusFlags::STRICT_MEMORY_METERING;
        let mut strict_store =
            WasmerRunner::create_engine_with_consensus_flags(MAX_PAGES, strict_flags).unwrap();
        let strict_module = Module::new(&strict_store, &wasm).unwrap();
        let err = Instance::new(&mut strict_store, &strict_module, &imports! {}).unwrap_err();

        assert!(
            err.to_string().contains("Table minimum exceeds"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn strict_validation_rejects_large_active_data_segment() {
        let data = "x".repeat(MAX_MEMORY_COPY_SIZE as usize + 1);
        let wasm = wat::parse_str(format!(
            r#"
            (module
              (memory 17)
              (data (i32.const 0) "{data}")
            )
            "#
        ))
        .unwrap();

        let err = WasmerRunner::validate_bytecode_for_strict_memory_metering(&wasm, MAX_PAGES)
            .unwrap_err();

        assert!(
            err.to_string().contains("data segment exceeds"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn strict_validation_rejects_large_passive_data_segment() {
        let data = "x".repeat(MAX_MEMORY_COPY_SIZE as usize + 1);
        let wasm = wat::parse_str(format!(
            r#"
            (module
              (memory 17)
              (data "{data}")
            )
            "#
        ))
        .unwrap();

        let err = WasmerRunner::validate_bytecode_for_strict_memory_metering(&wasm, MAX_PAGES)
            .unwrap_err();

        assert!(
            err.to_string().contains("data segment exceeds"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn strict_validation_rejects_aggregate_element_segments_above_limit() {
        let first_segment = " $f".repeat((MAX_TABLE_ELEMENTS / 2) as usize);
        let second_segment =
            " $f".repeat((MAX_TABLE_ELEMENTS - (MAX_TABLE_ELEMENTS / 2) + 1) as usize);
        let wasm = wat::parse_str(format!(
            r#"
            (module
              (func $f)
              (elem declare func{first_segment})
              (elem declare func{second_segment})
            )
            "#
        ))
        .unwrap();

        let err = WasmerRunner::validate_bytecode_for_strict_memory_metering(&wasm, MAX_PAGES)
            .unwrap_err();

        assert!(
            err.to_string().contains("element sections exceed"),
            "unexpected error: {err}"
        );
    }
}
