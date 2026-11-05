#![no_std]
//! Legacy program startup adapter for the canonical Kernel Client.

extern crate alloc;

pub mod boot;
mod allocator;

pub use kernel_client::{lock, memory, room, unit};
