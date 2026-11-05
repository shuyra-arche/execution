#![cfg_attr(target_arch = "riscv64", no_std)]
#![cfg_attr(target_arch = "riscv64", no_main)]

extern crate alloc;

#[cfg_attr(target_arch = "riscv64", mold::entry)]
fn main() {
    let _: fn(execution::memory::Reference) -> kernel_client::memory::Reference = |value| value;
    let _: fn(execution::memory::View) -> kernel_client::memory::View = |value| value;
    let _: fn(execution::unit::task::Join<usize>) -> kernel_client::unit::task::Join<usize> =
        |value| value;
    let _: fn(kernel_client::TeamId, usize, &[usize], usize)
        -> kernel_client::UnitResult<kernel_client::TaskId> = execution::unit::spawn;

    let report = execution::boot::Report::note(kernel_client::EXIT_OK, "legacy entry");
    assert_eq!(report.parts(), (kernel_client::EXIT_OK, Some("legacy entry")));
    let value = core::hint::black_box(alloc::boxed::Box::new(42usize));
    assert_eq!(*value, 42);
}
