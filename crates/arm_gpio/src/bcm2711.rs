//! Types and definitions for bcm2711 GPIO registers.
//!
//! The official documentation: https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf

use core::ptr::NonNull;

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

use log::{debug, trace};

const GPIO_REGS_BASE_ADDRESS: *mut usize = 0x7e200000 as *mut usize;

register_structs! {
    /// GPIO registers.
    GPIORegs {
        (0x00 => _reserved1),
        (0x04 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x08 => _reserved2),
        (0x94 => GPPUD: ReadWrite<u32, GPPUD::Register>),
        (0x98 => GPPUDCLK0: ReadWrite<u32, GPPUDCLK0::Register>),
        (0x9C => _reserved3),
        (0xE4 => GPIO_PUP_PDN_CNTRL_REG0: ReadWrite<u32, GPIO_PUP_PDN_CNTRL_REG0::Register>),
        (0xE8 => @END),
    }
}

register_bitfields! {
    u32,

/// GPIO Function Select 1
    GPFSEL1 [
        /// Pin 15
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100  // PL011 UART RX
        ],

        /// Pin 14
        FSEL14 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100  // PL011 UART TX
        ]
    ],

    /// GPIO Pull-up / Pull-down Register 0
    ///
    /// BCM2711 only.
    GPIO_PUP_PDN_CNTRL_REG0 [
        /// Pin 15
        GPIO_PUP_PDN_CNTRL15 OFFSET(30) NUMBITS(2) [
            NoResistor = 0b00,
            PullUp = 0b01
        ],

        /// Pin 14
        GPIO_PUP_PDN_CNTRL14 OFFSET(28) NUMBITS(2) [
            NoResistor = 0b00,
            PullUp = 0b01
        ]
    ]
}

pub struct GPIO {
    base: NonNull<GPIORegs>,
}

unsafe impl Send for GPIO {}
unsafe impl Sync for GPIO {}

impl GPIO {
    pub const fn new() -> Self {
        Self {
            base: NonNull::new(GPIO_REGS_BASE_ADDRESS).unwrap().cast(),
        }
    }

    const fn regs(&self) -> &GPIORegs {
        unsafe { self.base.as_ref() }
    }

    /// Initializes the Pl011 UART.
    ///
    /// It clears all irqs, sets fifo trigger level, enables rx interrupt, enables receives
    pub fn init(&mut self) {
        trace!("init");
        todo!()
    }

    /// enable pins gpio_input
    pub fn enable_pin_input(&mut self, c: u8) {
        trace!("input {c}");
        todo!()
    }

    /// enable pins gpio output
    pub fn enable_pin_output(&mut self, c: u8) {
        trace!("output {c}");
        todo!()
    }

    // /// Output a char c to data register
    // pub fn putchar(&mut self, c: u8) {
    //     while self.regs().fr.get() & (1 << 5) != 0 {}
    //     self.regs().dr.set(c as u32);
    // }
    //
    // /// Return a byte if pl011 has received, or it will return `None`.
    // pub fn getchar(&mut self) -> Option<u8> {
    //     if self.regs().fr.get() & (1 << 4) == 0 {
    //         Some(self.regs().dr.get() as u8)
    //     } else {
    //         None
    //     }
    // }
    //
    // /// Return true if pl011 has received an interrupt
    // pub fn is_receive_interrupt(&self) -> bool {
    //     let pending = self.regs().mis.get();
    //     pending & (1 << 4) != 0
    // }
    //
    // /// Clear all interrupts
    // pub fn ack_interrupts(&mut self) {
    //     self.regs().icr.set(0x7ff);
    // }
}
