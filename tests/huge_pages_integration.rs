//! Integration test for huge pages with Wasmer
use op_vm::domain::vm::{
    HugePageConfig, HugePageLinearMemory,
    get_huge_page_linear_stats, reset_huge_page_linear_stats,
    verify_huge_pages_in_use,
};
use wasmer::sys::vm::LinearMemory;
use wasmer_types::{MemoryStyle, MemoryType, Pages};

#[test]
fn test_huge_pages_with_wasmer_memory() {
    println!("\n=== Testing Huge Pages with Wasmer-compatible Memory ===\n");

    reset_huge_page_linear_stats();

    // Create a 32MB memory (512 wasm pages)
    let memory_type = MemoryType {
        minimum: Pages(512),
        maximum: Some(Pages(512)),
        shared: false,
    };

    let style = MemoryStyle::Static {
        bound: Pages(512),
        offset_guard_size: 0,
    };

    let config = HugePageConfig {
        use_explicit_huge_pages: true,
        use_transparent_huge_pages: true,
        log_fallback_warnings: true,
        use_mlock: true,
    };

    let mem = HugePageLinearMemory::new(&memory_type, &style, config)
        .expect("Failed to create memory");

    println!("Memory size: {} pages ({} MB)", mem.size().0, mem.size().0 * 64 / 1024);
    println!("Uses MAP_HUGETLB: {}", mem.uses_huge_pages());

    let (allocs, fallbacks) = get_huge_page_linear_stats();
    println!("Explicit allocs: {}, Fallbacks: {}", allocs, fallbacks);

    // Verify with smaps
    let vm_def = mem.vmmemory();
    let (ptr, size) = unsafe {
        let def = vm_def.as_ref();
        (def.base as *mut u8, def.current_length)
    };

    if let Ok(using_huge) = verify_huge_pages_in_use(ptr, size) {
        println!("/proc/self/smaps confirms huge pages: {}", using_huge);
    }

    // Write and read test
    unsafe {
        let slice = std::slice::from_raw_parts_mut(ptr, size);
        slice[0] = 42;
        slice[size - 1] = 255;
        assert_eq!(slice[0], 42);
        assert_eq!(slice[size - 1], 255);
    }
    println!("Memory read/write: OK");

    assert!(mem.uses_huge_pages() || fallbacks > 0);
    println!("\n✓ Test passed!\n");
}

#[test]
#[ignore]
fn benchmark_huge_pages_vs_regular() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║   HUGE PAGES vs REGULAR PAGES BENCHMARK                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    const SIZE_MB: usize = 32;
    const WASM_PAGES: u32 = (SIZE_MB * 1024 / 64) as u32;
    const ITERATIONS: usize = 100;
    const RANDOM_ACCESSES: usize = 100_000;

    let memory_type = MemoryType {
        minimum: Pages(WASM_PAGES),
        maximum: Some(Pages(WASM_PAGES)),
        shared: false,
    };

    let style = MemoryStyle::Static {
        bound: Pages(WASM_PAGES),
        offset_guard_size: 0,
    };

    // Benchmark with regular pages
    println!("--- Regular Pages (4KB) ---");
    let config_regular = HugePageConfig::disabled();
    let mem_regular = HugePageLinearMemory::new(&memory_type, &style, config_regular)
        .expect("Failed to create regular memory");

    let regular_results = benchmark_memory(&mem_regular, ITERATIONS, RANDOM_ACCESSES);

    // Benchmark with huge pages
    println!("\n--- Huge Pages (2MB) ---");
    let config_huge = HugePageConfig::default();
    let mem_huge = HugePageLinearMemory::new(&memory_type, &style, config_huge)
        .expect("Failed to create huge page memory");

    println!("Using MAP_HUGETLB: {}", mem_huge.uses_huge_pages());

    let huge_results = benchmark_memory(&mem_huge, ITERATIONS, RANDOM_ACCESSES);

    // Print comparison
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║   RESULTS                                                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("Memory size: {} MB ({} wasm pages)", SIZE_MB, WASM_PAGES);
    println!("TLB entries needed (4KB): {}", SIZE_MB * 1024 * 1024 / 4096);
    println!("TLB entries needed (2MB): {}", SIZE_MB * 1024 * 1024 / (2 * 1024 * 1024));

    println!("\n{:<25} {:>12} {:>12} {:>10}", "Operation", "Regular", "Huge", "Speedup");
    println!("{:-<25} {:->12} {:->12} {:->10}", "", "", "", "");

    print_comparison("Sequential Write", regular_results.0, huge_results.0);
    print_comparison("Sequential Read", regular_results.1, huge_results.1);
    print_comparison("Random Write", regular_results.2, huge_results.2);
    print_comparison("Random Read", regular_results.3, huge_results.3);
}

fn benchmark_memory(mem: &HugePageLinearMemory, iterations: usize, random_accesses: usize) -> (f64, f64, f64, f64) {
    let vm_def = mem.vmmemory();
    let (ptr, size) = unsafe {
        let def = vm_def.as_ref();
        (def.base as *mut u8, def.current_length)
    };

    let slice = unsafe { std::slice::from_raw_parts_mut(ptr, size) };

    // Sequential write
    let start = std::time::Instant::now();
    for iter in 0..iterations {
        let byte = (iter & 0xFF) as u8;
        for i in (0..size).step_by(64) {
            slice[i] = byte;
        }
    }
    let seq_write = start.elapsed().as_secs_f64() * 1000.0;
    println!("Sequential write: {:.2} ms", seq_write);

    // Sequential read
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for _ in 0..iterations {
        for i in (0..size).step_by(64) {
            sum = sum.wrapping_add(slice[i] as u64);
        }
    }
    std::hint::black_box(sum);
    let seq_read = start.elapsed().as_secs_f64() * 1000.0;
    println!("Sequential read: {:.2} ms", seq_read);

    // Random write
    let mut rng = SimpleRng::new(12345);
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        for _ in 0..random_accesses {
            let idx = rng.next_usize(size);
            slice[idx] = (idx & 0xFF) as u8;
        }
    }
    let rand_write = start.elapsed().as_secs_f64() * 1000.0;
    println!("Random write: {:.2} ms", rand_write);

    // Random read
    let mut rng = SimpleRng::new(12345);
    let start = std::time::Instant::now();
    let mut sum: u64 = 0;
    for _ in 0..iterations {
        for _ in 0..random_accesses {
            let idx = rng.next_usize(size);
            sum = sum.wrapping_add(slice[idx] as u64);
        }
    }
    std::hint::black_box(sum);
    let rand_read = start.elapsed().as_secs_f64() * 1000.0;
    println!("Random read: {:.2} ms", rand_read);

    (seq_write, seq_read, rand_write, rand_read)
}

fn print_comparison(name: &str, regular: f64, huge: f64) {
    let speedup = regular / huge;
    let indicator = if speedup > 1.05 { "faster" } else if speedup < 0.95 { "slower" } else { "same" };
    println!("{:<25} {:>10.2}ms {:>10.2}ms {:>8.2}x {}", name, regular, huge, speedup, indicator);
}

struct SimpleRng { state: u64 }
impl SimpleRng {
    fn new(seed: u64) -> Self { Self { state: seed } }
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }
    fn next_usize(&mut self, max: usize) -> usize { (self.next() as usize) % max }
}
