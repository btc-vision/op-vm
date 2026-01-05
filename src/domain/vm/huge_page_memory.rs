//! Huge page memory support for WebAssembly linear memory.
//!
//! This module provides huge page (2MB) support for Wasmer's linear memory allocation
//! to reduce TLB pressure and improve memory access performance.
//!
//! # Overview
//!
//! For a 32MB contract, using 2MB huge pages requires only 16 TLB entries
//! compared to 8192 entries with 4KB pages, significantly reducing TLB misses.
//!
//! # Strategies
//!
//! - **Explicit Huge Pages**: Uses `MAP_HUGETLB` flag with mmap. Requires pre-allocated
//!   huge pages in the system. Falls back to regular pages if unavailable.
//!
//! - **Transparent Huge Pages (THP)**: Uses `madvise(MADV_HUGEPAGE)` to hint the kernel.
//!   Less deterministic but doesn't require system configuration.

use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

/// 2MB huge page size
pub const HUGE_PAGE_SIZE: usize = 2 * 1024 * 1024;

/// 4KB regular page size
pub const REGULAR_PAGE_SIZE: usize = 4 * 1024;

/// Statistics for huge page allocations
static HUGE_PAGE_ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
static HUGE_PAGE_FALLBACKS: AtomicUsize = AtomicUsize::new(0);
static HUGE_PAGE_MADVISE_SUCCESS: AtomicUsize = AtomicUsize::new(0);
static HUGE_PAGE_MADVISE_FAILED: AtomicUsize = AtomicUsize::new(0);

/// Configuration for huge page memory allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HugePageConfig {
    /// Whether to attempt explicit huge page allocation with MAP_HUGETLB
    pub use_explicit_huge_pages: bool,
    /// Whether to use madvise(MADV_HUGEPAGE) for transparent huge pages
    pub use_transparent_huge_pages: bool,
    /// Whether to log warnings when falling back to regular pages
    pub log_fallback_warnings: bool,
    /// Whether to use mlock() to lock pages in RAM (prevents swapping)
    pub use_mlock: bool,
}

impl Default for HugePageConfig {
    fn default() -> Self {
        Self {
            use_explicit_huge_pages: true,
            use_transparent_huge_pages: true,
            log_fallback_warnings: true,
            use_mlock: true,
        }
    }
}

impl HugePageConfig {
    /// Create a config that only uses explicit huge pages
    pub fn explicit_only() -> Self {
        Self {
            use_explicit_huge_pages: true,
            use_transparent_huge_pages: false,
            log_fallback_warnings: true,
            use_mlock: true,
        }
    }

    /// Create a config that only uses transparent huge pages
    pub fn transparent_only() -> Self {
        Self {
            use_explicit_huge_pages: false,
            use_transparent_huge_pages: true,
            log_fallback_warnings: true,
            use_mlock: true,
        }
    }

    /// Create a config that disables huge pages entirely
    pub fn disabled() -> Self {
        Self {
            use_explicit_huge_pages: false,
            use_transparent_huge_pages: false,
            log_fallback_warnings: false,
            use_mlock: false,
        }
    }
}

/// Result of a huge page allocation attempt
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HugePageAllocationResult {
    /// Successfully allocated with explicit huge pages (MAP_HUGETLB)
    ExplicitHugePages,
    /// Successfully applied madvise(MADV_HUGEPAGE) for transparent huge pages
    TransparentHugePages,
    /// Fell back to regular 4KB pages
    RegularPages,
    /// Memory region is too small for huge pages (< 2MB)
    TooSmallForHugePages,
}

/// Statistics about huge page usage
#[derive(Debug, Clone, Copy)]
pub struct HugePageStats {
    /// Number of successful explicit huge page allocations
    pub explicit_allocations: usize,
    /// Number of fallbacks to regular pages
    pub fallbacks: usize,
    /// Number of successful madvise calls
    pub madvise_success: usize,
    /// Number of failed madvise calls
    pub madvise_failed: usize,
}

impl HugePageStats {
    /// Get current statistics
    pub fn get() -> Self {
        Self {
            explicit_allocations: HUGE_PAGE_ALLOCATIONS.load(Ordering::Relaxed),
            fallbacks: HUGE_PAGE_FALLBACKS.load(Ordering::Relaxed),
            madvise_success: HUGE_PAGE_MADVISE_SUCCESS.load(Ordering::Relaxed),
            madvise_failed: HUGE_PAGE_MADVISE_FAILED.load(Ordering::Relaxed),
        }
    }

    /// Reset all statistics to zero
    pub fn reset() {
        HUGE_PAGE_ALLOCATIONS.store(0, Ordering::Relaxed);
        HUGE_PAGE_FALLBACKS.store(0, Ordering::Relaxed);
        HUGE_PAGE_MADVISE_SUCCESS.store(0, Ordering::Relaxed);
        HUGE_PAGE_MADVISE_FAILED.store(0, Ordering::Relaxed);
    }
}

/// Raw memory allocation with huge page support
#[derive(Debug)]
pub struct HugePageMemory {
    ptr: NonNull<u8>,
    size: usize,
    is_huge_page: bool,
}

// SAFETY: The memory is owned by this struct and properly aligned
unsafe impl Send for HugePageMemory {}
unsafe impl Sync for HugePageMemory {}

impl HugePageMemory {
    /// Allocate memory with huge page support
    ///
    /// # Arguments
    /// * `size` - The size in bytes to allocate (will be rounded up to page boundary)
    /// * `config` - Configuration for huge page allocation
    ///
    /// # Returns
    /// A tuple of (HugePageMemory, HugePageAllocationResult)
    #[cfg(target_os = "linux")]
    pub fn new(size: usize, config: &HugePageConfig) -> Result<(Self, HugePageAllocationResult), String> {
        if size == 0 {
            return Err("Cannot allocate zero-sized memory".to_string());
        }

        // If size is less than one huge page, don't bother with huge pages
        // We check the original size, not aligned, to avoid wasting memory
        if size < HUGE_PAGE_SIZE {
            return Self::allocate_regular(size, config)
                .map(|mem| (mem, HugePageAllocationResult::TooSmallForHugePages));
        }

        // Round up to huge page boundary for huge page allocations
        let aligned_size = round_up_to_huge_page(size);

        // Try explicit huge pages first if configured
        if config.use_explicit_huge_pages {
            match Self::allocate_explicit_huge_pages(aligned_size) {
                Ok(mem) => {
                    HUGE_PAGE_ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
                    return Ok((mem, HugePageAllocationResult::ExplicitHugePages));
                }
                Err(e) => {
                    HUGE_PAGE_FALLBACKS.fetch_add(1, Ordering::Relaxed);
                    if config.log_fallback_warnings {
                        log::warn!(
                            "Failed to allocate {} bytes with explicit huge pages: {}. Falling back.",
                            aligned_size, e
                        );
                    }
                }
            }
        }

        // Allocate regular memory
        let mem = Self::allocate_regular(aligned_size, config)?;

        // Try to apply madvise for transparent huge pages if configured
        if config.use_transparent_huge_pages {
            match Self::apply_madvise_hugepage(mem.ptr.as_ptr(), aligned_size) {
                Ok(()) => {
                    HUGE_PAGE_MADVISE_SUCCESS.fetch_add(1, Ordering::Relaxed);
                    return Ok((mem, HugePageAllocationResult::TransparentHugePages));
                }
                Err(e) => {
                    HUGE_PAGE_MADVISE_FAILED.fetch_add(1, Ordering::Relaxed);
                    if config.log_fallback_warnings {
                        log::warn!(
                            "madvise(MADV_HUGEPAGE) failed for {} bytes: {}. Using regular pages.",
                            aligned_size, e
                        );
                    }
                }
            }
        }

        Ok((mem, HugePageAllocationResult::RegularPages))
    }

    /// Non-Linux platforms: always use regular allocation
    #[cfg(not(target_os = "linux"))]
    pub fn new(size: usize, config: &HugePageConfig) -> Result<(Self, HugePageAllocationResult), String> {
        if size == 0 {
            return Err("Cannot allocate zero-sized memory".to_string());
        }
        Self::allocate_regular(size, config)
            .map(|mem| (mem, HugePageAllocationResult::RegularPages))
    }

    /// Allocate memory using explicit huge pages (MAP_HUGETLB)
    #[cfg(target_os = "linux")]
    fn allocate_explicit_huge_pages(size: usize) -> Result<Self, String> {
        use libc::{mmap, MAP_ANONYMOUS, MAP_FAILED, MAP_HUGETLB, MAP_PRIVATE, PROT_READ, PROT_WRITE};
        use std::ptr;

        let ptr = unsafe {
            mmap(
                ptr::null_mut(),
                size,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS | MAP_HUGETLB,
                -1,
                0,
            )
        };

        if ptr == MAP_FAILED {
            let err = std::io::Error::last_os_error();
            return Err(format!("mmap with MAP_HUGETLB failed: {}", err));
        }

        Ok(Self {
            ptr: NonNull::new(ptr as *mut u8).unwrap(),
            size,
            is_huge_page: true,
        })
    }

    /// Allocate regular memory without huge pages
    #[cfg(target_os = "linux")]
    fn allocate_regular(size: usize, _config: &HugePageConfig) -> Result<Self, String> {
        use libc::{mmap, MAP_ANONYMOUS, MAP_FAILED, MAP_PRIVATE, PROT_READ, PROT_WRITE};
        use std::ptr;

        let aligned_size = round_up_to_page(size);

        let ptr = unsafe {
            mmap(
                ptr::null_mut(),
                aligned_size,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            )
        };

        if ptr == MAP_FAILED {
            let err = std::io::Error::last_os_error();
            return Err(format!("mmap failed: {}", err));
        }

        Ok(Self {
            ptr: NonNull::new(ptr as *mut u8).unwrap(),
            size: aligned_size,
            is_huge_page: false,
        })
    }

    /// Non-Linux regular allocation
    #[cfg(not(target_os = "linux"))]
    fn allocate_regular(size: usize, _config: &HugePageConfig) -> Result<Self, String> {
        let aligned_size = round_up_to_page(size);
        let layout = std::alloc::Layout::from_size_align(aligned_size, REGULAR_PAGE_SIZE)
            .map_err(|e| format!("Invalid layout: {}", e))?;

        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            return Err("Failed to allocate memory".to_string());
        }

        Ok(Self {
            ptr: NonNull::new(ptr).unwrap(),
            size: aligned_size,
            is_huge_page: false,
        })
    }

    /// Apply madvise(MADV_HUGEPAGE) to request transparent huge pages
    #[cfg(target_os = "linux")]
    fn apply_madvise_hugepage(ptr: *mut u8, size: usize) -> Result<(), String> {
        // MADV_HUGEPAGE = 14 on Linux
        const MADV_HUGEPAGE: i32 = 14;

        let result = unsafe { libc::madvise(ptr as *mut libc::c_void, size, MADV_HUGEPAGE) };

        if result != 0 {
            let err = std::io::Error::last_os_error();
            return Err(format!("madvise(MADV_HUGEPAGE) failed: {}", err));
        }

        Ok(())
    }

    /// Get a pointer to the allocated memory
    #[inline]
    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    /// Get the size of the allocated memory
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    /// Check if this allocation uses huge pages
    #[inline]
    pub fn is_huge_page(&self) -> bool {
        self.is_huge_page
    }

    /// Get a slice of the memory
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.size) }
    }

    /// Get a mutable slice of the memory
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.size) }
    }
}

impl Drop for HugePageMemory {
    fn drop(&mut self) {
        if self.size > 0 {
            #[cfg(target_os = "linux")]
            {
                unsafe {
                    libc::munmap(self.ptr.as_ptr() as *mut libc::c_void, self.size);
                }
            }
            #[cfg(not(target_os = "linux"))]
            {
                let layout = std::alloc::Layout::from_size_align(self.size, REGULAR_PAGE_SIZE)
                    .expect("Invalid layout in drop");
                unsafe {
                    std::alloc::dealloc(self.ptr.as_ptr(), layout);
                }
            }
        }
    }
}

/// Round size up to the nearest page boundary
#[inline]
pub fn round_up_to_page(size: usize) -> usize {
    (size + REGULAR_PAGE_SIZE - 1) & !(REGULAR_PAGE_SIZE - 1)
}

/// Round size up to the nearest huge page boundary
#[inline]
pub fn round_up_to_huge_page(size: usize) -> usize {
    (size + HUGE_PAGE_SIZE - 1) & !(HUGE_PAGE_SIZE - 1)
}

/// Apply madvise(MADV_HUGEPAGE) to an existing memory region
///
/// This can be used to request transparent huge pages for memory
/// that was already allocated by Wasmer.
///
/// # Safety
/// The caller must ensure that `ptr` points to a valid memory region
/// of at least `size` bytes that was allocated with mmap.
#[cfg(target_os = "linux")]
pub unsafe fn apply_hugepage_hint(ptr: *mut u8, size: usize) -> Result<(), String> {
    if size < HUGE_PAGE_SIZE {
        return Err("Memory region too small for huge pages".to_string());
    }

    // Align to huge page boundary
    let aligned_ptr = ((ptr as usize + HUGE_PAGE_SIZE - 1) & !(HUGE_PAGE_SIZE - 1)) as *mut u8;
    let offset = aligned_ptr as usize - ptr as usize;

    if offset >= size {
        return Err("Cannot align to huge page boundary within region".to_string());
    }

    let aligned_size = (size - offset) & !(HUGE_PAGE_SIZE - 1);
    if aligned_size == 0 {
        return Err("Aligned region too small for huge pages".to_string());
    }

    const MADV_HUGEPAGE: i32 = 14;
    let result = libc::madvise(aligned_ptr as *mut libc::c_void, aligned_size, MADV_HUGEPAGE);

    if result != 0 {
        let err = std::io::Error::last_os_error();
        return Err(format!("madvise(MADV_HUGEPAGE) failed: {}", err));
    }

    HUGE_PAGE_MADVISE_SUCCESS.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub unsafe fn apply_hugepage_hint(_ptr: *mut u8, _size: usize) -> Result<(), String> {
    Err("Huge pages are only supported on Linux".to_string())
}

/// Check if the system has huge pages available
#[cfg(target_os = "linux")]
pub fn check_huge_pages_available() -> Result<usize, String> {
    use std::fs;

    let content = fs::read_to_string("/proc/sys/vm/nr_hugepages")
        .map_err(|e| format!("Failed to read nr_hugepages: {}", e))?;

    let nr_hugepages: usize = content
        .trim()
        .parse()
        .map_err(|e| format!("Failed to parse nr_hugepages: {}", e))?;

    if nr_hugepages == 0 {
        return Err("No huge pages configured. Run: echo N > /proc/sys/vm/nr_hugepages".to_string());
    }

    Ok(nr_hugepages)
}

#[cfg(not(target_os = "linux"))]
pub fn check_huge_pages_available() -> Result<usize, String> {
    Err("Huge pages are only supported on Linux".to_string())
}

/// Get the number of free huge pages available
#[cfg(target_os = "linux")]
pub fn get_free_huge_pages() -> Result<usize, String> {
    use std::fs;

    let content = fs::read_to_string("/proc/sys/vm/nr_hugepages")
        .map_err(|e| format!("Failed to read nr_hugepages: {}", e))?;
    let total: usize = content.trim().parse().unwrap_or(0);

    let content = fs::read_to_string("/proc/meminfo")
        .map_err(|e| format!("Failed to read meminfo: {}", e))?;

    for line in content.lines() {
        if line.starts_with("HugePages_Free:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                return parts[1]
                    .parse()
                    .map_err(|e| format!("Failed to parse HugePages_Free: {}", e));
            }
        }
    }

    Ok(total)
}

#[cfg(not(target_os = "linux"))]
pub fn get_free_huge_pages() -> Result<usize, String> {
    Err("Huge pages are only supported on Linux".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_up_to_page() {
        assert_eq!(round_up_to_page(0), 0);
        assert_eq!(round_up_to_page(1), REGULAR_PAGE_SIZE);
        assert_eq!(round_up_to_page(4096), REGULAR_PAGE_SIZE);
        assert_eq!(round_up_to_page(4097), 2 * REGULAR_PAGE_SIZE);
    }

    #[test]
    fn test_round_up_to_huge_page() {
        assert_eq!(round_up_to_huge_page(0), 0);
        assert_eq!(round_up_to_huge_page(1), HUGE_PAGE_SIZE);
        assert_eq!(round_up_to_huge_page(HUGE_PAGE_SIZE), HUGE_PAGE_SIZE);
        assert_eq!(round_up_to_huge_page(HUGE_PAGE_SIZE + 1), 2 * HUGE_PAGE_SIZE);
    }

    #[test]
    fn test_huge_page_config_default() {
        let config = HugePageConfig::default();
        assert!(config.use_explicit_huge_pages);
        assert!(config.use_transparent_huge_pages);
        assert!(config.log_fallback_warnings);
    }

    #[test]
    fn test_allocate_regular_memory() {
        let config = HugePageConfig::disabled();
        let size = 64 * 1024; // 64KB

        let result = HugePageMemory::new(size, &config);
        assert!(result.is_ok());

        let (mem, alloc_result) = result.unwrap();
        assert!(mem.size() >= size);

        // On non-Linux or with disabled config, should get regular pages
        #[cfg(not(target_os = "linux"))]
        assert_eq!(alloc_result, HugePageAllocationResult::RegularPages);
    }

    #[test]
    fn test_memory_write_read() {
        let config = HugePageConfig::disabled();
        let size = 4096;

        let (mut mem, _) = HugePageMemory::new(size, &config).unwrap();

        // Write some data
        let slice = mem.as_mut_slice();
        slice[0] = 42;
        slice[100] = 123;
        slice[4095] = 255;

        // Read it back
        let slice = mem.as_slice();
        assert_eq!(slice[0], 42);
        assert_eq!(slice[100], 123);
        assert_eq!(slice[4095], 255);
    }

    #[test]
    fn test_stats() {
        HugePageStats::reset();
        let stats = HugePageStats::get();
        assert_eq!(stats.explicit_allocations, 0);
        assert_eq!(stats.fallbacks, 0);
    }

    #[test]
    fn test_small_allocation_skips_huge_pages() {
        let config = HugePageConfig::default();
        let size = 4096; // 4KB - too small for huge pages

        let result = HugePageMemory::new(size, &config);
        assert!(result.is_ok());

        let (_, alloc_result) = result.unwrap();
        // Small allocations should skip huge pages
        assert!(matches!(
            alloc_result,
            HugePageAllocationResult::TooSmallForHugePages | HugePageAllocationResult::RegularPages
        ));
    }
}
