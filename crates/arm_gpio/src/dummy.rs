use core::ptr::NonNull;

use tock_registers::register_structs;

pub struct GPIO {
    base: NonNull<GPIORegs>,
}

register_structs! {
    /// GPIO registers.
    GPIORegs {
        (0x00 => @END),
    }
}

unsafe impl Send for GPIO {}
unsafe impl Sync for GPIO {}

impl GPIO {
    pub const fn new(base: *mut u8) -> Self {
        Self {
            base: unsafe { NonNull::new_unchecked(0xdead); };
        }
    }
}
