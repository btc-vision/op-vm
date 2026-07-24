use std::ptr::NonNull;
use wasmer::sys::vm;
use wasmer::sys::vm::{VMConfig, VMMemoryDefinition, VMTableDefinition};
use wasmer_compiler::Tunables;
use wasmer_types::{
    MemoryError, MemoryStyle, MemoryType, Pages, TableStyle, TableType, WASM_PAGE_SIZE,
};

use crate::domain::runner::MAX_TABLE_ELEMENTS;

#[cfg(target_os = "windows")]
const DYNAMIC_MEMORY_GUARD_SIZE: u64 = 2 * WASM_PAGE_SIZE as u64;
#[cfg(not(target_os = "windows"))]
const DYNAMIC_MEMORY_GUARD_SIZE: u64 = WASM_PAGE_SIZE as u64;

/// A custom tunables that allows you to set a memory limit.
///
/// After adjusting the memory limits, it delegates all other logic
/// to the base tunables.
pub struct LimitingTunables<T: Tunables> {
    /// The maximum a linear memory is allowed to be (in Wasm pages, 64 KiB each).
    /// Since Wasmer ensures there is only none or one memory, this is practically
    /// an upper limit for the guest memory.
    max_pages: Pages, // 1 page = 64 KiB
    vm_config: VMConfig,
    /// The base implementation we delegate all the logic to
    base: T,
    strict_memory_metering: bool,
}

impl<T: Tunables> LimitingTunables<T> {
    pub fn new(base: T, max_pages: u32, stack_size: usize, strict_memory_metering: bool) -> Self {
        let vm_config = VMConfig {
            wasm_stack_size: Some(stack_size),
        };

        Self {
            max_pages: Pages(max_pages),
            vm_config,
            base,
            strict_memory_metering,
        }
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

    fn adjust_table(&self, requested: &TableType) -> TableType {
        let mut adjusted = *requested;
        if requested.maximum.is_none() {
            adjusted.maximum = Some(MAX_TABLE_ELEMENTS);
        }
        adjusted
    }

    fn validate_table(&self, ty: &TableType) -> Result<(), String> {
        if ty.minimum > MAX_TABLE_ELEMENTS {
            return Err("Table minimum exceeds the allowed table limit".to_string());
        }

        if let Some(max) = ty.maximum {
            if max > MAX_TABLE_ELEMENTS {
                return Err("Table maximum exceeds the allowed table limit".to_string());
            }

            if max < ty.minimum {
                return Err("Table maximum is smaller than table minimum".to_string());
            }
        } else {
            return Err("Table maximum unset".to_string());
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
        if self.strict_memory_metering {
            MemoryStyle::Dynamic {
                offset_guard_size: DYNAMIC_MEMORY_GUARD_SIZE,
            }
        } else {
            self.base.memory_style(&adjusted)
        }
    }

    /// Construct a `TableStyle` for the provided `TableType`
    ///
    /// Delegated to base.
    fn table_style(&self, table: &TableType) -> TableStyle {
        if self.strict_memory_metering {
            let adjusted = self.adjust_table(table);
            self.base.table_style(&adjusted)
        } else {
            self.base.table_style(table)
        }
    }

    /// Create a memory owned by the host given a [`MemoryType`] and a [`MemoryStyle`].
    ///
    /// The requested memory type is validated, adjusted to the limited and then passed to base.
    fn create_host_memory(
        &self,
        ty: &MemoryType,
        style: &MemoryStyle,
    ) -> Result<vm::VMMemory, MemoryError> {
        let adjusted = self.adjust_memory(ty);
        self.validate_memory(&adjusted)?;
        self.base.create_host_memory(&adjusted, style)
    }

    /// Create a memory owned by the VM given a [`MemoryType`] and a [`MemoryStyle`].
    ///
    /// Delegated to base.
    unsafe fn create_vm_memory(
        &self,
        ty: &MemoryType,
        style: &MemoryStyle,
        vm_definition_location: NonNull<VMMemoryDefinition>,
    ) -> Result<vm::VMMemory, MemoryError> {
        let adjusted = self.adjust_memory(ty);
        self.validate_memory(&adjusted)?;
        self.base
            .create_vm_memory(&adjusted, style, vm_definition_location)
    }

    /// Create a table owned by the host given a [`TableType`] and a [`TableStyle`].
    ///
    /// Delegated to base.
    fn create_host_table(&self, ty: &TableType, style: &TableStyle) -> Result<vm::VMTable, String> {
        if self.strict_memory_metering {
            let adjusted = self.adjust_table(ty);
            self.validate_table(&adjusted)?;
            self.base.create_host_table(&adjusted, style)
        } else {
            self.base.create_host_table(ty, style)
        }
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
        if self.strict_memory_metering {
            let adjusted = self.adjust_table(ty);
            self.validate_table(&adjusted)?;
            self.base
                .create_vm_table(&adjusted, style, vm_definition_location)
        } else {
            self.base.create_vm_table(ty, style, vm_definition_location)
        }
    }

    fn vmconfig(&self) -> &VMConfig {
        &self.vm_config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::runner::{MAX_PAGES, STACK_SIZE};
    use wasmer::sys::BaseTunables;
    use wasmer_types::target::Target;

    #[test]
    fn roswell_memory_style_proves_static_4gib_reservation_was_real() {
        let base = BaseTunables::for_target(&Target::default());
        let tunables = LimitingTunables::new(base, MAX_PAGES, STACK_SIZE, false);
        let memory = MemoryType::new(Pages(1), Some(Pages(MAX_PAGES)), false);

        let style = tunables.memory_style(&memory);

        match style {
            MemoryStyle::Static { bound, .. } => {
                assert_eq!(bound.0, 0x10000);
                assert_eq!(
                    bound.0 as u64 * WASM_PAGE_SIZE as u64,
                    4 * 1024 * 1024 * 1024
                );
                assert!(
                    bound.0 > MAX_PAGES,
                    "Roswell delegates to Wasmer's static bound instead of the VM page cap"
                );
            }
            other => panic!("expected Roswell static style, got {other:?}"),
        }
    }

    #[test]
    fn strict_memory_style_uses_dynamic_style_instead_of_4gib_static_bound() {
        let base = BaseTunables::for_target(&Target::default());
        let tunables = LimitingTunables::new(base, MAX_PAGES, STACK_SIZE, true);
        let memory = MemoryType::new(Pages(1), Some(Pages(MAX_PAGES)), false);

        let style = tunables.memory_style(&memory);

        match style {
            MemoryStyle::Dynamic { offset_guard_size } => {
                assert_eq!(offset_guard_size, DYNAMIC_MEMORY_GUARD_SIZE);
            }
            other => panic!("expected strict dynamic style, got {other:?}"),
        }
    }

    #[test]
    fn strict_table_validation_rejects_unbounded_or_oversized_tables() {
        let base = BaseTunables::for_target(&Target::default());
        let tunables = LimitingTunables::new(base, MAX_PAGES, STACK_SIZE, true);

        let oversized_min =
            TableType::new(wasmer_types::Type::FuncRef, MAX_TABLE_ELEMENTS + 1, None);
        let err = tunables
            .validate_table(&tunables.adjust_table(&oversized_min))
            .unwrap_err();
        assert!(err.contains("minimum exceeds"));

        let oversized_max =
            TableType::new(wasmer_types::Type::FuncRef, 1, Some(MAX_TABLE_ELEMENTS + 1));
        let err = tunables
            .validate_table(&tunables.adjust_table(&oversized_max))
            .unwrap_err();
        assert!(err.contains("maximum exceeds"));
    }
}
