//! Common traits and types for graphics display device drivers.

#![no_std]
#![feature(allocator_api)]
#![feature(strict_provenance)]

extern crate alloc;
pub(crate) mod dma;
pub mod host;
use core::alloc::Allocator;

#[doc(no_inline)]
pub use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};
use log::info;


