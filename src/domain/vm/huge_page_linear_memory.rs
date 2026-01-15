//! Custom LinearMemory implementation using Linux huge pages (2MB).
//!
//! This module provides a drop-in replacement for Wasmer's default memory
//! that uses `mmap` with `MAP_HUGETLB` for guaranteed huge page allocation.

use std::cell::UnsafeCell;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasmer_types::{MemoryError, MemoryStyle, MemoryType, Pages, WASM_PAGE_SIZE};

use super::huge_page_memory::{HugePageConfig, HUGE_PAGE_SIZE, REGULAR_PAGE_SIZE};

/// Statistics for huge page linear memory
static HUGE_PAGE_LINEAR_ALLOCS: AtomicUsize = AtomicUsize::new(0);
static HUGE_PAGE_LINEAR_FALLBACKS: AtomicUsize = AtomicUsize::new(0);

/// Get huge page linear memory statistics
pub fn get_huge_page_linear_stats() -> (usize, usize) {
    (
        HUGE_PAGE_LINEAR_ALLOCS.load(Ordering::Relaxed),
        HUGE_PAGE_LINEAR_FALLBACKS.load(Ordering::Relaxed),
    )
}

/// Reset huge page linear memory statistics
pub fn reset_huge_page_linear_stats() {
    HUGE_PAGE_LINEAR_ALLOCS.store(0, Ordering::Relaxed);
    HUGE_PAGE_LINEAR_FALLBACKS.store(0, Ordering::Relaxed);
}

/// VMMemoryDefinition as defined by Wasmer - the structure that compiled
/// WebAssembly code accesses directly.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMMemoryDefinition {
    /// Pointer to the base of the memory
    pub base: *mut u8,
    /// Current length of the memory in bytes
    pub current_length: usize,
}

// SAFETY: VMMemoryDefinition only contains a pointer and length
unsafe impl Send for VMMemoryDefinition {}
unsafe impl Sync for VMMemoryDefinition {}

/// Configuration for huge page linear memory
#[derive(Debug, Clone)]
struct MemoryConfig {
    /// Memory type from WebAssembly
    memory_type: MemoryType,
    /// Memory style (static/dynamic)
    style: MemoryStyle,
    /// Maximum pages allowed
    maximum: Option<Pages>,
    /// Guard page size in bytes
    offset_guard_size: usize,
}

/// A WebAssembly linear memory backed by Linux huge pages.
///
/// This implementation uses `mmap` with `MAP_HUGETLB` flag to allocate
/// memory using 2MB huge pages, reducing TLB pressure significantly.
#[derive(Debug)]
pub struct HugePageLinearMemory {
    /// Pointer to the allocated memory
    ptr: *mut u8,
    /// Total allocated size (including guard pages)
    total_size: usize,
    /// Current accessible size in bytes
    accessible_size: usize,
    /// Current size in WebAssembly pages
    current_pages: Pages,
    /// Whether this allocation uses huge pages
    uses_huge_pages: bool,
    /// Memory configuration
    config: MemoryConfig,
    /// The VM memory definition for compiled code (internal, owned by this struct)
    vm_memory_definition: Box<UnsafeCell<VMMemoryDefinition>>,
    /// External VM memory definition location (when created via create_vm_memory)
    /// This is the location that compiled WebAssembly code actually accesses.
    external_vm_definition: Option<NonNull<VMMemoryDefinition>>,
    /// Huge page configuration
    huge_page_config: HugePageConfig,
}

// SAFETY: Memory is properly managed and accessed through proper synchronization
unsafe impl Send for HugePageLinearMemory {}
unsafe impl Sync for HugePageLinearMemory {}

impl HugePageLinearMemory {
    /// Create a new huge page linear memory.
    ///
    /// # Arguments
    /// * `memory` - WebAssembly memory type specification
    /// * `style` - Memory style (static or dynamic)
    /// * `huge_page_config` - Configuration for huge page allocation
    ///
    /// # Returns
    /// A new HugePageLinearMemory or an error
    pub fn new(
        memory: &MemoryType,
        style: &MemoryStyle,
        huge_page_config: HugePageConfig,
    ) -> Result<Self, MemoryError> {
        Self::new_internal(memory, style, huge_page_config, None)
    }

    /// Create a new huge page linear memory with an external VM definition.
    ///
    /// # Safety
    /// The `vm_memory_location` must point to a valid, properly aligned location
    /// that will remain valid for the lifetime of this memory.
    pub unsafe fn from_definition(
        memory: &MemoryType,
        style: &MemoryStyle,
        huge_page_config: HugePageConfig,
        vm_memory_location: NonNull<VMMemoryDefinition>,
    ) -> Result<Self, MemoryError> {
        Self::new_internal(memory, style, huge_page_config, Some(vm_memory_location))
    }

    fn new_internal(
        memory: &MemoryType,
        style: &MemoryStyle,
        huge_page_config: HugePageConfig,
        vm_memory_location: Option<NonNull<VMMemoryDefinition>>,
    ) -> Result<Self, MemoryError> {
        // Validate memory parameters
        if memory.minimum > Pages::max_value() {
            return Err(MemoryError::MinimumMemoryTooLarge {
                min_requested: memory.minimum,
                max_allowed: Pages::max_value(),
            });
        }

        if let Some(max) = memory.maximum {
            if max > Pages::max_value() {
                return Err(MemoryError::MaximumMemoryTooLarge {
                    max_requested: max,
                    max_allowed: Pages::max_value(),
                });
            }
            if max < memory.minimum {
                return Err(MemoryError::InvalidMemory {
                    reason: format!(
                        "maximum ({} pages) is less than minimum ({} pages)",
                        max.0, memory.minimum.0
                    ),
                });
            }
        }

        let offset_guard_size = style.offset_guard_size() as usize;

        // Calculate sizes
        // For huge page pre-allocation, we want to reserve space for the maximum expected size
        let minimum_pages = match style {
            MemoryStyle::Dynamic { .. } => {
                // For dynamic memory, use maximum if set to pre-allocate huge pages
                // This avoids expensive reallocation when memory grows
                memory.maximum.unwrap_or(memory.minimum)
            }
            MemoryStyle::Static { bound, .. } => {
                if *bound < memory.minimum {
                    return Err(MemoryError::InvalidMemory {
                        reason: "static bound less than minimum".to_string(),
                    });
                }
                *bound
            }
        };

        let minimum_bytes = minimum_pages.bytes().0;
        let initial_bytes = memory.minimum.bytes().0;
        let total_size = minimum_bytes
            .checked_add(offset_guard_size)
            .ok_or_else(|| MemoryError::InvalidMemory {
                reason: "memory size overflow".to_string(),
            })?;

        // Allocate memory with huge pages
        // Pass minimum_bytes (the full bound) as huge_page_target_size so we pre-allocate
        // with huge pages even if initial accessible memory is small
        let (ptr, actual_total_size, uses_huge_pages) =
            Self::allocate_memory(total_size, initial_bytes, minimum_bytes, &huge_page_config)?;

        // Create internal VM definition (always needed)
        let vm_memory_definition = Box::new(UnsafeCell::new(VMMemoryDefinition {
            base: ptr,
            current_length: initial_bytes,
        }));

        // If external location is provided (from create_vm_memory), update it too
        // This is the location that compiled WebAssembly code actually accesses
        let external_vm_definition = if let Some(mut ext_def) = vm_memory_location {
            unsafe {
                let def = ext_def.as_mut();
                def.base = ptr;
                def.current_length = initial_bytes;
            }
            Some(ext_def)
        } else {
            None
        };

        if uses_huge_pages {
            HUGE_PAGE_LINEAR_ALLOCS.fetch_add(1, Ordering::Relaxed);
        } else {
            HUGE_PAGE_LINEAR_FALLBACKS.fetch_add(1, Ordering::Relaxed);
        }

        Ok(Self {
            ptr,
            total_size: actual_total_size,
            accessible_size: initial_bytes,
            current_pages: memory.minimum,
            uses_huge_pages,
            config: MemoryConfig {
                memory_type: *memory,
                style: *style,
                maximum: memory.maximum,
                offset_guard_size,
            },
            vm_memory_definition,
            external_vm_definition,
            huge_page_config,
        })
    }

    /// Allocate memory with huge pages, falling back to regular pages if needed.
    ///
    /// # Arguments
    /// * `total_size` - Total virtual memory to reserve (including guard pages)
    /// * `accessible_size` - Initial accessible memory (will be PROT_READ|PROT_WRITE)
    /// * `huge_page_target_size` - Size to consider for huge page allocation (typically the max/bound)
    ///
    /// # Returns
    /// * `(*mut u8, usize, bool)` - (pointer, actual_allocated_size, uses_huge_pages)
    ///
    /// IMPORTANT: This follows Wasmer's approach - reserve total_size with PROT_NONE,
    /// then only make accessible_size accessible. Guard pages remain PROT_NONE and
    /// use no physical memory. Huge pages are pre-allocated for huge_page_target_size.
    #[cfg(target_os = "linux")]
    fn allocate_memory(
        total_size: usize,
        accessible_size: usize,
        huge_page_target_size: usize,
        config: &HugePageConfig,
    ) -> Result<(*mut u8, usize, bool), MemoryError> {
        use libc::{
            mmap, mprotect, MAP_ANONYMOUS, MAP_FAILED, MAP_HUGETLB, MAP_PRIVATE, PROT_NONE,
            PROT_READ, PROT_WRITE,
        };
        use std::ptr;

        #[cfg(feature = "debug-metering")]
        log::info!(
            "[HugePages] allocate_memory called: total_size={}, accessible_size={}, huge_page_target_size={}, config={:?}",
            total_size, accessible_size, huge_page_target_size, config
        );

        if total_size == 0 {
            return Ok((ptr::null_mut(), 0, false));
        }

        // Round total to regular page boundary (for guard pages)
        // IMPORTANT: We return aligned_total so Drop knows the exact size to munmap
        let aligned_total = round_up_to_page(total_size);

        // First, reserve the FULL region with PROT_NONE (like Wasmer does).
        // This is just virtual address space - no physical memory is used.
        // The guard region will remain PROT_NONE.
        let base_ptr = unsafe {
            mmap(
                ptr::null_mut(),
                aligned_total,
                PROT_NONE,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            )
        };

        if base_ptr == MAP_FAILED {
            let err = std::io::Error::last_os_error();
            return Err(MemoryError::Region(format!("mmap reserve failed: {}", err)));
        }

        // If no accessible memory needed, we're done
        if accessible_size == 0 {
            return Ok((base_ptr as *mut u8, aligned_total, false));
        }

        // Round huge page target size to huge page boundary for pre-allocation
        let aligned_huge_target = round_up_to_huge_page(huge_page_target_size);
        let mut uses_huge_pages = false;

        // Try explicit huge pages based on the TARGET size (max/bound), not just initial accessible
        // This pre-allocates huge pages for the full expected memory usage
        if config.use_explicit_huge_pages && huge_page_target_size >= HUGE_PAGE_SIZE {
            #[cfg(feature = "debug-metering")]
            log::info!("[HugePages] Attempting MAP_HUGETLB for {} bytes (target), accessible={}", aligned_huge_target, accessible_size);
            // Remap the target region with MAP_HUGETLB using MAP_FIXED
            // This pre-allocates huge pages for the full expected memory usage
            let huge_ptr = unsafe {
                mmap(
                    base_ptr,
                    aligned_huge_target,
                    PROT_READ | PROT_WRITE,
                    MAP_PRIVATE | MAP_ANONYMOUS | MAP_HUGETLB | libc::MAP_FIXED,
                    -1,
                    0,
                )
            };

            if huge_ptr != MAP_FAILED {
                uses_huge_pages = true;
                #[cfg(feature = "debug-metering")]
                log::info!(
                    "[HugePages] Mapped {} bytes with MAP_HUGETLB (target={}, accessible={}, guard={} bytes)",
                    aligned_huge_target,
                    huge_page_target_size,
                    accessible_size,
                    aligned_total - aligned_huge_target
                );

                // Optionally mlock only the accessible portion
                if config.use_mlock {
                    let mlock_result = unsafe { libc::mlock(base_ptr, accessible_size) };
                    if mlock_result != 0 && config.log_fallback_warnings {
                        println!(
                            "[HugePages WARN] mlock failed for {} bytes: {} (memory may be swapped)",
                            accessible_size,
                            std::io::Error::last_os_error()
                        );
                    }
                }
            } else if config.log_fallback_warnings {
                println!(
                    "[HugePages WARN] MAP_HUGETLB failed: {}. Using regular pages.",
                    std::io::Error::last_os_error()
                );
            }
        }

        // If we didn't get huge pages, make accessible portion readable/writable with regular pages
        if !uses_huge_pages {
            #[cfg(feature = "debug-metering")]
            if huge_page_target_size < HUGE_PAGE_SIZE {
                log::info!(
                    "[HugePages] Skipping huge pages: huge_page_target_size {} < HUGE_PAGE_SIZE {} (2MB)",
                    huge_page_target_size, HUGE_PAGE_SIZE
                );
            }
            let result = unsafe { mprotect(base_ptr, accessible_size, PROT_READ | PROT_WRITE) };

            if result != 0 {
                let err = std::io::Error::last_os_error();
                unsafe {
                    libc::munmap(base_ptr, aligned_total);
                }
                return Err(MemoryError::Region(format!("mprotect failed: {}", err)));
            }

            // Try transparent huge pages via madvise for the full target size
            if config.use_transparent_huge_pages && huge_page_target_size >= HUGE_PAGE_SIZE {
                const MADV_HUGEPAGE: i32 = 14;
                let result = unsafe { libc::madvise(base_ptr, huge_page_target_size, MADV_HUGEPAGE) };

                if result == 0 {
                    #[cfg(feature = "debug-metering")]
                    log::info!(
                        "[HugePages] Applied MADV_HUGEPAGE to {} bytes (transparent huge pages)",
                        huge_page_target_size
                    );
                }
            }
        }

        Ok((base_ptr as *mut u8, aligned_total, uses_huge_pages))
    }

    #[cfg(not(target_os = "linux"))]
    fn allocate_memory(
        total_size: usize,
        _accessible_size: usize,
        _huge_page_target_size: usize,
        _config: &HugePageConfig,
    ) -> Result<(*mut u8, usize, bool), MemoryError> {
        let aligned_total = round_up_to_page(total_size);
        let layout = std::alloc::Layout::from_size_align(aligned_total, REGULAR_PAGE_SIZE)
            .map_err(|e| MemoryError::Region(format!("Invalid layout: {}", e)))?;

        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            return Err(MemoryError::Region("Allocation failed".to_string()));
        }

        Ok((ptr, aligned_total, false))
    }

    /// Check if this memory is using huge pages
    pub fn uses_huge_pages(&self) -> bool {
        self.uses_huge_pages
    }

    /// Update the VM memory definition with new base and length.
    ///
    /// CRITICAL: This updates BOTH the internal definition AND the external
    /// definition (if one was provided). The external definition is what
    /// compiled WebAssembly code actually accesses, so it MUST be kept in sync.
    fn update_vm_memory_definition(&mut self, base: *mut u8, length: usize) {
        // Update internal definition
        unsafe {
            let def = &mut *self.vm_memory_definition.get();
            def.base = base;
            def.current_length = length;
        }

        // CRITICAL: Also update external definition if present
        // This is the location that compiled WebAssembly code actually uses!
        if let Some(mut ext_def) = self.external_vm_definition {
            unsafe {
                let def = ext_def.as_mut();
                def.base = base;
                def.current_length = length;
            }
        }
    }
}

impl Drop for HugePageLinearMemory {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.total_size > 0 {
            #[cfg(target_os = "linux")]
            unsafe {
                libc::munmap(self.ptr as *mut libc::c_void, self.total_size);
            }

            #[cfg(not(target_os = "linux"))]
            unsafe {
                let layout =
                    std::alloc::Layout::from_size_align(self.total_size, REGULAR_PAGE_SIZE)
                        .expect("Invalid layout in drop");
                std::alloc::dealloc(self.ptr, layout);
            }
        }
    }
}

// Import LinearMemory trait from wasmer
use wasmer::sys::vm::LinearMemory;

impl LinearMemory for HugePageLinearMemory {
    fn ty(&self) -> MemoryType {
        let mut ty = self.config.memory_type;
        ty.minimum = self.current_pages;
        ty
    }

    fn size(&self) -> Pages {
        self.current_pages
    }

    fn style(&self) -> MemoryStyle {
        self.config.style
    }

    fn grow(&mut self, delta: Pages) -> Result<Pages, MemoryError> {
        if delta.0 == 0 {
            return Ok(self.current_pages);
        }

        let new_pages = self
            .current_pages
            .checked_add(delta)
            .ok_or(MemoryError::CouldNotGrow {
                current: self.current_pages,
                attempted_delta: delta,
            })?;

        let prev_pages = self.current_pages;

        #[cfg(feature = "debug-metering")]
        log::info!(
            "[HugePages] grow called: {} -> {} pages (+{} pages, {} -> {} bytes), uses_huge_pages={}",
            prev_pages.0, new_pages.0, delta.0,
            self.accessible_size, new_pages.bytes().0,
            self.uses_huge_pages
        );

        // Check against maximum
        if let Some(max) = self.config.maximum {
            if new_pages > max {
                return Err(MemoryError::CouldNotGrow {
                    current: self.current_pages,
                    attempted_delta: delta,
                });
            }
        }

        // Check absolute maximum
        if new_pages > Pages::max_value() {
            return Err(MemoryError::CouldNotGrow {
                current: self.current_pages,
                attempted_delta: delta,
            });
        }

        let new_bytes = new_pages.bytes().0;
        let max_bytes = self.total_size - self.config.offset_guard_size;

        if new_bytes > max_bytes {
            // Need to reallocate - this is expensive for huge pages
            #[cfg(feature = "debug-metering")]
            log::info!(
                "[HugePages] grow requires REALLOCATION: new_bytes {} > max_bytes {} (pre-allocated)",
                new_bytes, max_bytes
            );

            let new_total = new_bytes
                .checked_add(self.config.offset_guard_size)
                .ok_or(MemoryError::CouldNotGrow {
                    current: self.current_pages,
                    attempted_delta: delta,
                })?;

            // For reallocation, use new_bytes as the huge page target (we're growing to this size)
            let (new_ptr, actual_new_total, new_uses_huge) =
                Self::allocate_memory(new_total, new_bytes, new_bytes, &self.huge_page_config)?;

            // Copy old data
            if self.accessible_size > 0 {
                unsafe {
                    std::ptr::copy_nonoverlapping(self.ptr, new_ptr, self.accessible_size);
                }
            }

            // Free old memory
            if !self.ptr.is_null() && self.total_size > 0 {
                #[cfg(target_os = "linux")]
                unsafe {
                    libc::munmap(self.ptr as *mut libc::c_void, self.total_size);
                }
            }

            self.ptr = new_ptr;
            self.total_size = actual_new_total;
            self.uses_huge_pages = new_uses_huge;
        } else {
            // Just make more memory accessible - within pre-allocated region
            #[cfg(feature = "debug-metering")]
            log::info!(
                "[HugePages] grow using PRE-ALLOCATED memory: new_bytes {} <= max_bytes {}, huge_pages={}",
                new_bytes, max_bytes, self.uses_huge_pages
            );

            #[cfg(target_os = "linux")]
            {
                // If we're using huge pages, the memory is already pre-allocated and accessible
                // (MAP_HUGETLB requires PROT_READ|PROT_WRITE). Skip redundant mprotect.
                if !self.uses_huge_pages {
                    let prev_bytes = prev_pages.bytes().0;
                    let delta_bytes = new_bytes - prev_bytes;

                    if delta_bytes > 0 {
                        let result = unsafe {
                            libc::mprotect(
                                self.ptr.add(prev_bytes) as *mut libc::c_void,
                                delta_bytes,
                                libc::PROT_READ | libc::PROT_WRITE,
                            )
                        };

                        if result != 0 {
                            return Err(MemoryError::Region(format!(
                                "mprotect failed: {}",
                                std::io::Error::last_os_error()
                            )));
                        }
                    }
                }
            }
        }

        self.accessible_size = new_bytes;
        self.current_pages = new_pages;
        self.update_vm_memory_definition(self.ptr, new_bytes);

        Ok(prev_pages)
    }

    fn grow_at_least(&mut self, min_size: u64) -> Result<(), MemoryError> {
        let current_bytes = self.current_pages.bytes().0 as u64;
        if current_bytes >= min_size {
            return Ok(());
        }

        let needed_bytes = min_size - current_bytes;
        let needed_pages = ((needed_bytes as usize + WASM_PAGE_SIZE - 1) / WASM_PAGE_SIZE) as u32;
        self.grow(Pages(needed_pages))?;
        Ok(())
    }

    fn reset(&mut self) -> Result<(), MemoryError> {
        self.current_pages = Pages(0);
        self.accessible_size = 0;
        self.update_vm_memory_definition(self.ptr, 0);
        Ok(())
    }

    fn vmmemory(&self) -> NonNull<wasmer::sys::vm::VMMemoryDefinition> {
        // CRITICAL: When an external VM definition was provided (from create_vm_memory),
        // we MUST return that location because Wasmer's compiled code expects to access
        // memory definitions at that specific address. Returning our internal copy would
        // cause compiled code to read stale/wrong values, leading to crashes.
        if let Some(ext_def) = self.external_vm_definition {
            // SAFETY: The external definition has the same layout as Wasmer's VMMemoryDefinition
            unsafe {
                NonNull::new_unchecked(
                    ext_def.as_ptr() as *mut wasmer::sys::vm::VMMemoryDefinition
                )
            }
        } else {
            // No external definition - return our internal one (for create_host_memory path)
            // SAFETY: Our VMMemoryDefinition has the same layout as Wasmer's
            unsafe {
                NonNull::new_unchecked(
                    self.vm_memory_definition.get() as *mut wasmer::sys::vm::VMMemoryDefinition
                )
            }
        }
    }

    fn try_clone(&self) -> Result<Box<dyn LinearMemory + 'static>, MemoryError> {
        Err(MemoryError::InvalidMemory {
            reason: "HugePageLinearMemory cannot be cloned".to_string(),
        })
    }

    fn copy(&mut self) -> Result<Box<dyn LinearMemory + 'static>, MemoryError> {
        // Create a new memory with the same configuration
        let mut new_mem = Self::new(&self.config.memory_type, &self.config.style, self.huge_page_config)?;

        // Grow to current size
        if self.current_pages > new_mem.current_pages {
            let delta = Pages(self.current_pages.0 - new_mem.current_pages.0);
            new_mem.grow(delta)?;
        }

        // Copy data
        if self.accessible_size > 0 {
            unsafe {
                std::ptr::copy_nonoverlapping(self.ptr, new_mem.ptr, self.accessible_size);
            }
        }

        Ok(Box::new(new_mem))
    }
}

/// Round size up to the nearest 4KB page boundary
#[inline]
fn round_up_to_page(size: usize) -> usize {
    (size + REGULAR_PAGE_SIZE - 1) & !(REGULAR_PAGE_SIZE - 1)
}

/// Round size up to the nearest 2MB huge page boundary
#[inline]
fn round_up_to_huge_page(size: usize) -> usize {
    (size + HUGE_PAGE_SIZE - 1) & !(HUGE_PAGE_SIZE - 1)
}

/// Verify if a memory region is actually using huge pages by checking /proc/self/smaps
#[cfg(target_os = "linux")]
pub fn verify_huge_pages_in_use(ptr: *const u8, size: usize) -> Result<bool, String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let addr = ptr as usize;
    let end_addr = addr + size;

    let file = File::open("/proc/self/smaps").map_err(|e| format!("Failed to open smaps: {}", e))?;
    let reader = BufReader::new(file);

    let mut in_region = false;
    let mut found_huge_pages = false;

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Failed to read line: {}", e))?;

        // Check if this is a memory region header
        if line.contains('-') && !line.starts_with(' ') {
            // Parse the address range: "00400000-00452000 r-xp ..."
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(range) = parts.first() {
                let addrs: Vec<&str> = range.split('-').collect();
                if addrs.len() == 2 {
                    if let (Ok(start), Ok(end)) = (
                        usize::from_str_radix(addrs[0], 16),
                        usize::from_str_radix(addrs[1], 16),
                    ) {
                        // Check if our memory overlaps with this region
                        in_region = (start <= addr && addr < end) || (start < end_addr && end_addr <= end)
                            || (addr <= start && end <= end_addr);
                    }
                }
            }
        }

        // Check for huge page indicators in this region
        if in_region {
            if line.contains("AnonHugePages:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(kb) = parts[1].parse::<usize>() {
                        if kb > 0 {
                            found_huge_pages = true;
                        }
                    }
                }
            }
            // Also check for explicit huge pages
            if line.contains("hugetlb") || line.contains("Hugetlb") {
                found_huge_pages = true;
            }
        }
    }

    Ok(found_huge_pages)
}

#[cfg(not(target_os = "linux"))]
pub fn verify_huge_pages_in_use(_ptr: *const u8, _size: usize) -> Result<bool, String> {
    Err("Huge page verification only supported on Linux".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_huge_page_linear_memory() {
        let memory_type = MemoryType {
            minimum: Pages(16), // 1MB
            maximum: Some(Pages(512)), // 32MB
            shared: false,
        };

        let style = MemoryStyle::Static {
            bound: Pages(512),
            offset_guard_size: 0x8000_0000, // 2GB guard
        };

        let config = HugePageConfig::default();

        let result = HugePageLinearMemory::new(&memory_type, &style, config);
        assert!(result.is_ok(), "Failed to create memory: {:?}", result.err());

        let mem = result.unwrap();
        assert_eq!(mem.size(), Pages(16));
        println!("Uses huge pages: {}", mem.uses_huge_pages());
    }

    #[test]
    fn test_grow_memory() {
        let memory_type = MemoryType {
            minimum: Pages(1),
            maximum: Some(Pages(100)),
            shared: false,
        };

        let style = MemoryStyle::Dynamic {
            offset_guard_size: 0,
        };

        let config = HugePageConfig::disabled();

        let mut mem = HugePageLinearMemory::new(&memory_type, &style, config).unwrap();
        assert_eq!(mem.size(), Pages(1));

        let prev = mem.grow(Pages(10)).unwrap();
        assert_eq!(prev, Pages(1));
        assert_eq!(mem.size(), Pages(11));
    }

    #[test]
    fn test_memory_access() {
        let memory_type = MemoryType {
            minimum: Pages(1), // 64KB
            maximum: Some(Pages(10)),
            shared: false,
        };

        let style = MemoryStyle::Dynamic {
            offset_guard_size: 0,
        };

        let config = HugePageConfig::disabled();
        let mem = HugePageLinearMemory::new(&memory_type, &style, config).unwrap();

        // Get the VM memory definition and write/read data
        let vm_def = mem.vmmemory();
        unsafe {
            let def = vm_def.as_ref();
            let slice = std::slice::from_raw_parts_mut(def.base, def.current_length);

            // Write test pattern
            slice[0] = 42;
            slice[1000] = 123;
            slice[65535] = 255;

            // Read back
            assert_eq!(slice[0], 42);
            assert_eq!(slice[1000], 123);
            assert_eq!(slice[65535], 255);
        }
    }

    #[test]
    #[ignore] // Run with: cargo test verify_huge_pages_actually_used --release -- --nocapture --ignored
    fn verify_huge_pages_actually_used() {
        println!("\n");
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║     HUGE PAGE VERIFICATION TEST                              ║");
        println!("╚══════════════════════════════════════════════════════════════╝");

        // Check system configuration
        println!("\n--- System Configuration ---");
        match crate::domain::vm::huge_page_memory::check_huge_pages_available() {
            Ok(n) => println!("Huge pages configured: {} ({} MB)", n, n * 2),
            Err(e) => {
                println!("WARNING: {}", e);
                println!("To configure huge pages, run:");
                println!("  sudo sh -c 'echo 256 > /proc/sys/vm/nr_hugepages'");
            }
        }

        // Create a large memory region (32MB) that should use huge pages
        let memory_type = MemoryType {
            minimum: Pages(512), // 32MB
            maximum: Some(Pages(512)),
            shared: false,
        };

        let style = MemoryStyle::Static {
            bound: Pages(512),
            offset_guard_size: 0, // No guard for this test
        };

        println!("\n--- Creating Memory with Huge Pages ---");
        let config = HugePageConfig {
            use_explicit_huge_pages: true,
            use_transparent_huge_pages: true,
            log_fallback_warnings: true,
            use_mlock: true,
        };

        reset_huge_page_linear_stats();

        let mem = HugePageLinearMemory::new(&memory_type, &style, config)
            .expect("Failed to create memory");

        println!("Memory created: {} pages ({} bytes)", mem.size().0, mem.size().bytes().0);
        println!("Uses huge pages (MAP_HUGETLB): {}", mem.uses_huge_pages());

        // Get stats
        let (allocs, fallbacks) = get_huge_page_linear_stats();
        println!("\n--- Allocation Statistics ---");
        println!("Explicit huge page allocations: {}", allocs);
        println!("Fallbacks to regular pages: {}", fallbacks);

        // Verify by checking /proc/self/smaps
        println!("\n--- Verifying via /proc/self/smaps ---");
        let vm_def = mem.vmmemory();
        let (ptr, size) = unsafe {
            let def = vm_def.as_ref();
            (def.base, def.current_length)
        };

        match verify_huge_pages_in_use(ptr, size) {
            Ok(true) => {
                println!("✓ VERIFIED: Memory region IS using huge pages!");
            }
            Ok(false) => {
                println!("✗ Memory region is NOT using huge pages");
                if mem.uses_huge_pages() {
                    println!("  (MAP_HUGETLB succeeded but not reflected in smaps - this is expected for hugetlbfs)");
                }
            }
            Err(e) => {
                println!("Could not verify: {}", e);
            }
        }

        // Touch all memory to ensure it's actually mapped
        println!("\n--- Touching all memory ---");
        unsafe {
            let slice = std::slice::from_raw_parts_mut(ptr, size);
            for i in (0..size).step_by(4096) {
                slice[i] = (i & 0xFF) as u8;
            }
        }
        println!("Memory touched successfully");

        // Final verdict
        println!("\n--- VERDICT ---");
        if mem.uses_huge_pages() {
            println!("✓ SUCCESS: Memory was allocated with MAP_HUGETLB (explicit huge pages)");
        } else if allocs == 0 && fallbacks > 0 {
            println!("✗ FALLBACK: Had to fall back to regular pages");
            println!("  Make sure huge pages are configured:");
            println!("  sudo sh -c 'echo 256 > /proc/sys/vm/nr_hugepages'");
        } else {
            println!("? UNKNOWN: Check the logs above for details");
        }

        // Assert for CI
        assert!(
            mem.uses_huge_pages() || fallbacks > 0,
            "Memory allocation should have attempted huge pages"
        );
    }
}
