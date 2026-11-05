//! Register the canonical program heap for the legacy startup adapter.

#[cfg(target_arch = "riscv64")]
#[global_allocator]
static HEAP: kernel_client::memory::HeapAllocator = kernel_client::memory::HeapAllocator;
