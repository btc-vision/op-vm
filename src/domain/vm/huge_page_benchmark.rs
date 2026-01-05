//! Benchmarks for comparing huge page vs regular page memory performance.
//!
//! Run with: `cargo test huge_page_benchmark --release -- --nocapture --ignored`
//!
//! Note: For accurate results, run with huge pages configured:
//! ```bash
//! # Reserve 256 huge pages (512MB)
//! sudo echo 256 > /proc/sys/vm/nr_hugepages
//! ```

#[cfg(test)]
mod tests {
    use crate::domain::vm::huge_page_memory::{
        check_huge_pages_available, get_free_huge_pages, HugePageAllocationResult, HugePageConfig,
        HugePageMemory, HugePageStats, HUGE_PAGE_SIZE,
    };
    use std::time::{Duration, Instant};

    const KB: usize = 1024;
    const MB: usize = 1024 * KB;

    /// Benchmark configuration
    struct BenchConfig {
        /// Memory size to allocate
        size: usize,
        /// Number of iterations for each test
        iterations: usize,
        /// Number of memory accesses per iteration
        accesses_per_iter: usize,
    }

    impl Default for BenchConfig {
        fn default() -> Self {
            Self {
                size: 32 * MB, // 32MB - typical contract size
                iterations: 100,
                accesses_per_iter: 10000,
            }
        }
    }

    /// Results from a benchmark run
    #[derive(Debug, Clone)]
    struct BenchResult {
        name: String,
        allocation_time: Duration,
        sequential_read_time: Duration,
        sequential_write_time: Duration,
        random_read_time: Duration,
        random_write_time: Duration,
        allocation_result: HugePageAllocationResult,
    }

    impl BenchResult {
        fn print(&self) {
            println!("\n=== {} ===", self.name);
            println!("Allocation result: {:?}", self.allocation_result);
            println!("Allocation time:      {:>10.3} ms", self.allocation_time.as_secs_f64() * 1000.0);
            println!("Sequential read:      {:>10.3} ms", self.sequential_read_time.as_secs_f64() * 1000.0);
            println!("Sequential write:     {:>10.3} ms", self.sequential_write_time.as_secs_f64() * 1000.0);
            println!("Random read:          {:>10.3} ms", self.random_read_time.as_secs_f64() * 1000.0);
            println!("Random write:         {:>10.3} ms", self.random_write_time.as_secs_f64() * 1000.0);
        }
    }

    /// Simple LCG random number generator for reproducible benchmarks
    struct SimpleRng {
        state: u64,
    }

    impl SimpleRng {
        fn new(seed: u64) -> Self {
            Self { state: seed }
        }

        fn next(&mut self) -> u64 {
            // LCG parameters from Numerical Recipes
            self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
            self.state
        }

        fn next_usize(&mut self, max: usize) -> usize {
            (self.next() as usize) % max
        }
    }

    /// Run benchmark with a specific configuration
    fn run_benchmark(
        name: &str,
        config: &BenchConfig,
        huge_page_config: HugePageConfig,
    ) -> Result<BenchResult, String> {
        // Measure allocation time
        let alloc_start = Instant::now();
        let (mut mem, alloc_result) = HugePageMemory::new(config.size, &huge_page_config)?;
        let allocation_time = alloc_start.elapsed();

        let slice = mem.as_mut_slice();
        let len = slice.len();

        // Sequential write benchmark
        let seq_write_start = Instant::now();
        for iter in 0..config.iterations {
            let byte = (iter & 0xFF) as u8;
            for i in (0..len).step_by(64) {
                // Write every cache line
                slice[i] = byte;
            }
        }
        let sequential_write_time = seq_write_start.elapsed();

        // Sequential read benchmark
        let seq_read_start = Instant::now();
        let mut sum: u64 = 0;
        for _ in 0..config.iterations {
            for i in (0..len).step_by(64) {
                sum = sum.wrapping_add(slice[i] as u64);
            }
        }
        let sequential_read_time = seq_read_start.elapsed();
        // Prevent optimization
        std::hint::black_box(sum);

        // Random write benchmark
        let mut rng = SimpleRng::new(12345);
        let random_write_start = Instant::now();
        for _ in 0..config.iterations {
            for _ in 0..config.accesses_per_iter {
                let idx = rng.next_usize(len);
                slice[idx] = (idx & 0xFF) as u8;
            }
        }
        let random_write_time = random_write_start.elapsed();

        // Random read benchmark
        let mut rng = SimpleRng::new(12345); // Reset RNG for same pattern
        let random_read_start = Instant::now();
        let mut sum: u64 = 0;
        for _ in 0..config.iterations {
            for _ in 0..config.accesses_per_iter {
                let idx = rng.next_usize(len);
                sum = sum.wrapping_add(slice[idx] as u64);
            }
        }
        let random_read_time = random_read_start.elapsed();
        std::hint::black_box(sum);

        Ok(BenchResult {
            name: name.to_string(),
            allocation_time,
            sequential_read_time,
            sequential_write_time,
            random_read_time,
            random_write_time,
            allocation_result: alloc_result,
        })
    }

    fn print_comparison(regular: &BenchResult, huge: &BenchResult) {
        println!("\n=== COMPARISON (Huge Pages vs Regular) ===");

        let alloc_speedup = regular.allocation_time.as_secs_f64() / huge.allocation_time.as_secs_f64();
        let seq_read_speedup = regular.sequential_read_time.as_secs_f64() / huge.sequential_read_time.as_secs_f64();
        let seq_write_speedup = regular.sequential_write_time.as_secs_f64() / huge.sequential_write_time.as_secs_f64();
        let rand_read_speedup = regular.random_read_time.as_secs_f64() / huge.random_read_time.as_secs_f64();
        let rand_write_speedup = regular.random_write_time.as_secs_f64() / huge.random_write_time.as_secs_f64();

        println!("Allocation:       {:>6.2}x {}", alloc_speedup, if alloc_speedup > 1.0 { "faster" } else { "slower" });
        println!("Sequential read:  {:>6.2}x {}", seq_read_speedup, if seq_read_speedup > 1.0 { "faster" } else { "slower" });
        println!("Sequential write: {:>6.2}x {}", seq_write_speedup, if seq_write_speedup > 1.0 { "faster" } else { "slower" });
        println!("Random read:      {:>6.2}x {}", rand_read_speedup, if rand_read_speedup > 1.0 { "faster" } else { "slower" });
        println!("Random write:     {:>6.2}x {}", rand_write_speedup, if rand_write_speedup > 1.0 { "faster" } else { "slower" });
    }

    #[test]
    #[ignore] // Run manually with: cargo test huge_page_benchmark --release -- --nocapture --ignored
    fn huge_page_benchmark() {
        println!("\n");
        println!("╔════════════════════════════════════════════════════════════════╗");
        println!("║          HUGE PAGE MEMORY BENCHMARK                            ║");
        println!("╚════════════════════════════════════════════════════════════════╝");

        // Check huge page availability
        println!("\n--- System Configuration ---");
        match check_huge_pages_available() {
            Ok(count) => println!("Huge pages configured: {} ({} MB)", count, count * 2),
            Err(e) => println!("Huge pages not available: {}", e),
        }
        match get_free_huge_pages() {
            Ok(count) => println!("Free huge pages: {} ({} MB)", count, count * 2),
            Err(e) => println!("Could not get free huge pages: {}", e),
        }

        let config = BenchConfig::default();
        println!("\n--- Benchmark Configuration ---");
        println!("Memory size: {} MB", config.size / MB);
        println!("Iterations: {}", config.iterations);
        println!("Random accesses per iteration: {}", config.accesses_per_iter);
        println!("TLB entries needed (4KB pages): {}", config.size / 4096);
        println!("TLB entries needed (2MB pages): {}", config.size / HUGE_PAGE_SIZE);

        // Reset statistics
        HugePageStats::reset();

        // Run benchmark with regular pages
        println!("\n--- Running Regular Pages Benchmark ---");
        let regular_result = run_benchmark(
            "Regular Pages (4KB)",
            &config,
            HugePageConfig::disabled(),
        );

        // Run benchmark with huge pages
        println!("\n--- Running Huge Pages Benchmark ---");
        let huge_result = run_benchmark(
            "Huge Pages (2MB)",
            &config,
            HugePageConfig::default(),
        );

        // Print results
        match (&regular_result, &huge_result) {
            (Ok(regular), Ok(huge)) => {
                regular.print();
                huge.print();
                print_comparison(regular, huge);
            }
            (Err(e), _) => println!("Regular pages benchmark failed: {}", e),
            (_, Err(e)) => println!("Huge pages benchmark failed: {}", e),
        }

        // Print statistics
        let stats = HugePageStats::get();
        println!("\n--- Huge Page Statistics ---");
        println!("Explicit huge page allocations: {}", stats.explicit_allocations);
        println!("Fallbacks to regular pages: {}", stats.fallbacks);
        println!("Successful madvise calls: {}", stats.madvise_success);
        println!("Failed madvise calls: {}", stats.madvise_failed);
    }

    #[test]
    #[ignore]
    fn huge_page_benchmark_various_sizes() {
        println!("\n");
        println!("╔════════════════════════════════════════════════════════════════╗");
        println!("║          HUGE PAGE BENCHMARK - VARIOUS SIZES                   ║");
        println!("╚════════════════════════════════════════════════════════════════╝");

        let sizes = [
            (1 * MB, "1 MB"),
            (2 * MB, "2 MB"),
            (4 * MB, "4 MB"),
            (8 * MB, "8 MB"),
            (16 * MB, "16 MB"),
            (32 * MB, "32 MB"),
            (64 * MB, "64 MB"),
        ];

        for (size, name) in sizes {
            println!("\n\n=== Memory Size: {} ===", name);

            let config = BenchConfig {
                size,
                iterations: 50,
                accesses_per_iter: 5000,
            };

            let regular = run_benchmark(
                &format!("Regular {}", name),
                &config,
                HugePageConfig::disabled(),
            );

            let huge = run_benchmark(
                &format!("Huge Pages {}", name),
                &config,
                HugePageConfig::default(),
            );

            match (&regular, &huge) {
                (Ok(r), Ok(h)) => {
                    let rand_read_speedup = r.random_read_time.as_secs_f64() / h.random_read_time.as_secs_f64();
                    println!(
                        "Random read speedup: {:.2}x (Regular: {:.3}ms, Huge: {:.3}ms)",
                        rand_read_speedup,
                        r.random_read_time.as_secs_f64() * 1000.0,
                        h.random_read_time.as_secs_f64() * 1000.0
                    );
                }
                _ => println!("Benchmark failed for size {}", name),
            }
        }
    }

    #[test]
    fn test_huge_page_fallback() {
        // This test verifies that fallback to regular pages works
        let config = HugePageConfig {
            use_explicit_huge_pages: true,
            use_transparent_huge_pages: true,
            log_fallback_warnings: false, // Suppress warnings in test
            use_mlock: true,
        };

        // Try to allocate 4MB (2 huge pages worth)
        let result = HugePageMemory::new(4 * MB, &config);
        assert!(result.is_ok(), "Memory allocation should succeed with fallback");

        let (mem, alloc_result) = result.unwrap();
        assert!(mem.size() >= 4 * MB);

        // The allocation result tells us what happened
        println!("Allocation result: {:?}", alloc_result);

        // On systems without huge pages, this should fallback
        match alloc_result {
            HugePageAllocationResult::ExplicitHugePages => {
                println!("Explicit huge pages were used");
                assert!(mem.is_huge_page());
            }
            HugePageAllocationResult::TransparentHugePages => {
                println!("Transparent huge pages hint was applied");
            }
            HugePageAllocationResult::RegularPages => {
                println!("Fell back to regular pages (expected if no huge pages available)");
            }
            HugePageAllocationResult::TooSmallForHugePages => {
                panic!("4MB should be large enough for huge pages");
            }
        }
    }

    #[test]
    fn test_small_allocation_uses_regular_pages() {
        let config = HugePageConfig::default();

        // Allocate less than 2MB
        let result = HugePageMemory::new(1 * MB, &config);
        assert!(result.is_ok());

        let (mem, alloc_result) = result.unwrap();
        assert!(mem.size() >= 1 * MB);

        // Should use regular pages for small allocations
        assert!(
            matches!(
                alloc_result,
                HugePageAllocationResult::TooSmallForHugePages | HugePageAllocationResult::RegularPages
            ),
            "Small allocations should not use huge pages"
        );
    }
}
