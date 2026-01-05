use std::ptr::NonNull;
use wasmer::sys::vm;
use wasmer::sys::vm::{VMConfig, VMMemoryDefinition, VMTableDefinition};
use wasmer_compiler::Tunables;
use wasmer_types::{MemoryError, MemoryStyle, MemoryType, Pages, TableStyle, TableType};

use super::huge_page_linear_memory::HugePageLinearMemory;
use super::huge_page_memory::HugePageConfig;

/// A custom tunables that allows you to set a memory limit and use huge pages.
///
/// When huge pages are enabled, this tunables implementation creates
/// `HugePageLinearMemory` instances that use `mmap` with `MAP_HUGETLB`
/// for guaranteed 2MB huge page allocation.
pub struct LimitingTunables<T: Tunables> {
    /// The maximum a linear memory is allowed to be (in Wasm pages, 64 KiB each).
    /// Since Wasmer ensures there is only none or one memory, this is practically
    /// an upper limit for the guest memory.
    max_pages: Pages, // 1 page = 64 KiB
    vm_config: VMConfig,
    /// Configuration for huge page allocation
    huge_page_config: HugePageConfig,
    /// Whether to use huge page linear memory
    use_huge_page_memory: bool,
    /// The base implementation we delegate all the logic to
    base: T,
}

impl<T: Tunables> LimitingTunables<T> {
    /// Create new LimitingTunables with default settings (huge pages disabled)
    pub fn new(base: T, max_pages: u32, stack_size: usize) -> Self {
        Self::with_huge_pages(base, max_pages, stack_size, HugePageConfig::disabled())
    }

    /// Create new LimitingTunables with huge page support.
    ///
    /// When `huge_page_config` has `use_explicit_huge_pages` enabled,
    /// this will create custom `HugePageLinearMemory` instances that
    /// use `MAP_HUGETLB` for guaranteed huge page allocation.
    pub fn with_huge_pages(
        base: T,
        max_pages: u32,
        stack_size: usize,
        huge_page_config: HugePageConfig,
    ) -> Self {
        let vm_config = VMConfig {
            wasm_stack_size: Some(stack_size),
        };

        // Only use custom huge page memory if explicit huge pages are enabled
        let use_huge_page_memory = huge_page_config.use_explicit_huge_pages
            || huge_page_config.use_transparent_huge_pages;

        Self {
            max_pages: Pages(max_pages),
            vm_config,
            huge_page_config,
            use_huge_page_memory,
            base,
        }
    }

    /// Get the huge page configuration
    pub fn huge_page_config(&self) -> &HugePageConfig {
        &self.huge_page_config
    }

    /// Check if huge page memory is being used
    pub fn uses_huge_page_memory(&self) -> bool {
        self.use_huge_page_memory
    }

    /// Takes an input memory type as requested by the guest and sets
    /// a maximum if missing. The resulting memory type is passed to
    /// the base implementation. Validation occurs later during memory
    /// creation in create_host_memory and create_vm_memory.
    fn adjust_memory(&self, requested: &MemoryType) -> MemoryType {
        let mut adjusted = requested.clone();
        if requested.maximum.is_none() {
            adjusted.maximum = Some(self.max_pages);
        }
        adjusted
    }

    /// Ensures that a given memory type does not exceed the memory limit.
    /// Call this after adjusting the memory.
    fn validate_memory(&self, ty: &MemoryType) -> Result<(), MemoryError> {
        if ty.minimum > self.max_pages {
            return Err(MemoryError::Generic(
                "Minimum exceeds the allowed memory limit".to_string(),
            ));
        }

        if let Some(max) = ty.maximum {
            if max > self.max_pages {
                return Err(MemoryError::Generic(
                    "Maximum exceeds the allowed memory limit".to_string(),
                ));
            }
        } else {
            return Err(MemoryError::Generic("Maximum unset".to_string()));
        }

        Ok(())
    }
}

impl<T: Tunables> Tunables for LimitingTunables<T> {
    /// Construct a `MemoryStyle` for the provided `MemoryType`
    ///
    /// Delegated to base.
    fn memory_style(&self, memory: &MemoryType) -> MemoryStyle {
        let adjusted = self.adjust_memory(memory);
        self.base.memory_style(&adjusted)
    }

    /// Construct a `TableStyle` for the provided `TableType`
    ///
    /// Delegated to base.
    fn table_style(&self, table: &TableType) -> TableStyle {
        self.base.table_style(table)
    }

    /// Create a memory owned by the host given a [`MemoryType`] and a [`MemoryStyle`].
    ///
    /// When huge pages are enabled, creates a HugePageLinearMemory.
    /// Otherwise, delegates to base.
    fn create_host_memory(
        &self,
        ty: &MemoryType,
        style: &MemoryStyle,
    ) -> Result<vm::VMMemory, MemoryError> {
        let adjusted = self.adjust_memory(ty);
        self.validate_memory(&adjusted)?;

        if self.use_huge_page_memory {
            // Create our custom huge page memory
            let memory = HugePageLinearMemory::new(&adjusted, style, self.huge_page_config)?;

            if memory.uses_huge_pages() {
                log::info!(
                    "Created host memory with HUGE PAGES: {} pages ({} bytes)",
                    adjusted.minimum.0,
                    adjusted.minimum.bytes().0
                );
            } else {
                log::debug!(
                    "Created host memory with regular pages (fallback): {} pages",
                    adjusted.minimum.0
                );
            }

            // Convert to VMMemory
            Ok(vm::VMMemory::from_custom(Box::new(memory)
                as Box<dyn vm::LinearMemory + 'static>))
        } else {
            self.base.create_host_memory(&adjusted, style)
        }
    }

    /// Create a memory owned by the VM given a [`MemoryType`] and a [`MemoryStyle`].
    ///
    /// When huge pages are enabled, creates a HugePageLinearMemory.
    /// Otherwise, delegates to base.
    unsafe fn create_vm_memory(
        &self,
        ty: &MemoryType,
        style: &MemoryStyle,
        vm_definition_location: NonNull<VMMemoryDefinition>,
    ) -> Result<vm::VMMemory, MemoryError> {
        let adjusted = self.adjust_memory(ty);
        self.validate_memory(&adjusted)?;

        if self.use_huge_page_memory {
            // Create our custom huge page memory with external VM definition
            let memory = HugePageLinearMemory::from_definition(
                &adjusted,
                style,
                self.huge_page_config,
                // SAFETY: We're converting between compatible VMMemoryDefinition types
                NonNull::new_unchecked(
                    vm_definition_location.as_ptr()
                        as *mut super::huge_page_linear_memory::VMMemoryDefinition,
                ),
            )?;

            if memory.uses_huge_pages() {
                log::info!(
                    "Created VM memory with HUGE PAGES (MAP_HUGETLB): {} pages ({} bytes)",
                    adjusted.minimum.0,
                    adjusted.minimum.bytes().0
                );
            } else {
                log::debug!(
                    "Created VM memory with regular pages (fallback): {} pages",
                    adjusted.minimum.0
                );
            }

            Ok(vm::VMMemory::from_custom(Box::new(memory)
                as Box<dyn vm::LinearMemory + 'static>))
        } else {
            self.base
                .create_vm_memory(&adjusted, style, vm_definition_location)
        }
    }

    /// Create a table owned by the host given a [`TableType`] and a [`TableStyle`].
    ///
    /// Delegated to base.
    fn create_host_table(&self, ty: &TableType, style: &TableStyle) -> Result<vm::VMTable, String> {
        self.base.create_host_table(ty, style)
    }

    /// Create a table owned by the VM given a [`TableType`] and a [`TableStyle`].
    ///
    /// Delegated to base.
    unsafe fn create_vm_table(
        &self,
        ty: &TableType,
        style: &TableStyle,
        vm_definition_location: NonNull<VMTableDefinition>,
    ) -> Result<vm::VMTable, String> {
        self.base.create_vm_table(ty, style, vm_definition_location)
    }

    fn vmconfig(&self) -> &VMConfig {
        &self.vm_config
    }
}
