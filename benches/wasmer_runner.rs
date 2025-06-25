extern crate op_vm;

use op_vm::domain::runner::{ContractRunner, WasmerRunner, MAX_GAS_WASM_INIT};
use op_vm::domain::vm::get_gas_cost;
use wat::parse_str;

const BASIC_WAT: &str = r#"
(module
    (memory (export "memory") 1)
    (func (export "start"))
    (func (export "execute") (param i32) (result i32)
        local.get 0)
)"#;

/// Helper: build a fresh runner with enough head-room.
fn runner_from_wat(wat: &str) -> WasmerRunner {
    let wasm = parse_str(wat).expect("invalid WAT");
    let env = dummy_custom_env();
    WasmerRunner::from_bytecode(&wasm, 0, MAX_GAS_WASM_INIT + 20_000, 32, env, false)
        .expect("runner instantiation failed")
}

#[test]
fn gas_consumption_is_deterministic() {
    let mut r1 = runner_from_wat(BASIC_WAT);
    let used_1 = r1
        .execute(vec![0; 4], MAX_GAS_WASM_INIT + 20_000)
        .expect("first run failed")
        .gas_used;

    // brand-new identical instance to avoid state contamination
    let mut r2 = runner_from_wat(BASIC_WAT);
    let used_2 = r2
        .execute(vec![0; 4], MAX_GAS_WASM_INIT + 20_000)
        .expect("second run failed")
        .gas_used;

    assert_eq!(
        used_1, used_2,
        "gas must be identical across identical executions (determinism)"
    );
}

/// Verifies that the runtime aborts *exactly* when gas is exhausted.
#[test]
fn out_of_gas_error_is_raised() {
    let mut r = runner_from_wat(BASIC_WAT);

    // Leave only 10 gas units available – below the guaranteed minimum
    let err = r
        .execute(vec![0; 4], MAX_GAS_WASM_INIT + 10)
        .expect_err("should have failed");

    assert!(
        err.to_string().contains("out of gas"),
        "must return deterministic 'out of gas' error"
    );
}

const OOM_WAT: &str = r#"
(module
    (memory (export "memory") 1 1)   ;; hard-cap at 1 page
    (func (export "start"))
    (func (export "execute")
        (memory.grow (i32.const 5))  ;; impossible: requests 5 extra pages
        drop)
)"#;

#[test]
fn memory_limit_enforced() {
    let mut r = runner_from_wat(OOM_WAT);
    let err = r
        .execute(vec![], MAX_GAS_WASM_INIT + 50_000)
        .expect_err("execution must fail");

    assert!(
        err.to_string().contains("out of memory"),
        "VM has to stop deterministically on OOM"
    );
}
#[test]
fn metering_matches_static_cost_model() {
    const ADD_WAT: &str = r#"
    (module
        (func (export "start"))
        (func (export "execute") (result i32)
            i32.const 1
            i32.const 1
            i32.add)
    )"#;

    let mut r = runner_from_wat(ADD_WAT);
    let before = r.get_remaining_gas();
    let _ = r
        .execute(vec![], MAX_GAS_WASM_INIT + 20_000)
        .expect("execution failed");
    let after = r.get_remaining_gas();

    let expected = get_gas_cost(&wasmer::wasmparser::Operator::I32Add);
    assert_eq!(
        before - after,
        expected,
        "gas counter must charge the static cost model exactly"
    );
}
