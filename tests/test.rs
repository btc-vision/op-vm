#![cfg(test)]

extern crate op_vm;

mod tests {
    use super::*;
    // pull `WasmerRunner`, `CustomEnv`, constants …
    use op_vm::domain::runner::{ContractRunner, WasmerRunner, MAX_GAS_WASM_INIT};

    use op_vm::domain::vm::dummy_custom_env;
    use wat::parse_str;
    // turn inline WAT into raw Wasm bytes

    /// Compile WAT → byte‑code → run → return gas used by `execute`.
    fn run_contract(wasm: &[u8], max_gas: u64) -> u64 {
        // we keep pages generous (256 × 64 KiB = 16 MiB) so memory is not clamped
        let max_pages = 256;
        let custom_env = dummy_custom_env();

        // ① instantiate
        let mut runner = WasmerRunner::from_bytecode(
            wasm, /*used_gas =*/ 0, max_gas, max_pages, custom_env,
            /*is_debug_mode =*/ false,
        )
        .expect("failed to create runner");

        // ② call the exported `execute` (empty calldata)
        runner
            .execute(Vec::new(), max_gas)
            .expect("contract execution failed");

        // ③ deterministic performance metric
        runner.get_used_gas()
    }

    /// Trivial contract that does *nothing* but return `0`.
    const LIGHT_WAT: &str = r#"
        (module
          (func (export "start"))
          (func (export "execute") (param i32) (result i32)
            i32.const 0))
    "#;

    /// Memory‑bound contract: writes 4 KiB (≈ 1 024 × 4‑byte stores)
    /// into linear memory each time `execute` is called.
    ///
    ///  – 4 KiB / 64 B = 64 cache‑lines
    ///  – With the tuned schedule each store costs one `mem_rw_cost(4)` = 5 000 gas
    ///    → 1 024 × 5 000 ≈ **5.12 M‑gas** per call (plus a few ALU ops).
    const HEAVY_WAT: &str = r#"
        (module
          (memory (export "memory") 1)

          (func (export "start"))

          (func (export "execute") (param i32) (result i32)
            (local $ptr i32)
            (local.set $ptr (i32.const 0))
            (loop $loop
              local.get $ptr
              i32.const 4096            ;; stop after 4 KiB
              i32.ge_u
              if (result i32)
                i32.const 0
                return
              end
              ;; store constant 123 at (0 + $ptr)
              i32.const 0
              local.get $ptr
              i32.add
              i32.const 123
              i32.store

              ;; ptr += 4
              local.get $ptr
              i32.const 4
              i32.add
              local.set $ptr
              br $loop))
    "#;

    /// Heavy memory traffic must consume **orders of magnitude** more gas
    /// than the trivial contract.
    #[test]
    fn heavy_memory_contract_burns_more_gas() {
        let light_wasm = parse_str(LIGHT_WAT).unwrap();
        let heavy_wasm = parse_str(HEAVY_WAT).unwrap();

        // give both contracts plenty of headroom
        let max_gas = 10_000_000;

        let gas_light = run_contract(&light_wasm, max_gas);
        let gas_heavy = run_contract(&heavy_wasm, max_gas);

        // Expect at least a 10× gap (usually ~70 000 : 5 200 000).
        assert!(
            gas_heavy > gas_light * 10,
            "heavy contract should use >10× gas (light = {}, heavy = {})",
            gas_light,
            gas_heavy
        );
    }

    /// Under a *tight* gas‑cap the memory‑bound contract must trap with
    /// "out of gas" whereas the trivial one still succeeds.
    #[test]
    fn heavy_contract_exhausts_gas_when_cap_is_low() {
        let heavy_wasm = parse_str(HEAVY_WAT).unwrap();
        let light_wasm = parse_str(LIGHT_WAT).unwrap();

        // Give just enough for init + a few thousand gas of execution.
        let low_gas_budget = MAX_GAS_WASM_INIT + 200_000;

        // Light contract should *still* succeed under that budget …
        let _gas_light = run_contract(&light_wasm, low_gas_budget);

        // … but the heavy one must fail.
        let max_pages = 256;
        let custom_env = dummy_custom_env();

        let mut runner = WasmerRunner::from_bytecode(
            &heavy_wasm,
            0,
            low_gas_budget,
            max_pages,
            custom_env,
            false,
        )
        .expect("instantiation must succeed (init gas covered)");

        let exec_result = runner.execute(Vec::new(), low_gas_budget);
        assert!(
            exec_result.is_err(),
            "heavy contract should run out of gas with low budget"
        );

        // Optional: check the error string contains our canonical message
        let msg = format!("{}", exec_result.err().unwrap());
        assert!(
            msg.contains("out of gas"),
            "unexpected error: {msg:?}; should contain 'out of gas'"
        );
    }
}
