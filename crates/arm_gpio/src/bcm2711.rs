//! Types and definitions for bcm2711 GPIO registers.
//!
//! The official documentation: https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf

use core::ptr::NonNull;

use tock_registers::{
    interfaces::{ReadWriteable, Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};

use log::{debug, trace};

// const GPIO_REGS_BASE_ADDRESS: *mut usize = 0x7e200000 as *mut usize;
const GPIO_REGS_BASE_ADDRESS: *mut usize = 0xffff_0000_fe20_0000 as *mut usize; // ?

register_structs! {
    #[allow(non_snake_case)]
    GPIORegs {
        (0x00 => GPFSEL0: ReadWrite<u32, GPFSEL0::Register>),   // GPIO Function Select
        (0x04 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x08 => GPFSEL2: ReadWrite<u32, GPFSEL2::Register>),
        (0x0c => GPFSEL3: ReadWrite<u32, GPFSEL3::Register>),
        (0x10 => GPFSEL4: ReadWrite<u32, GPFSEL4::Register>),
        (0x14 => GPFSEL5: ReadWrite<u32, GPFSEL5::Register>),
        (0x18 => _reserved),
        (0x1c => GPSET0: ReadWrite<u32, GPSET0::Register>),     // GPIO Pin Output Set
        (0x20 => GPSET1: ReadWrite<u32, GPSET1::Register>),
        (0x24 => _reserved1),
        (0x28 => GPCLR0: ReadWrite<u32, GPCLR0::Register>),     // GPIO Pin Output Clear
        (0x2c => GPCLR1: ReadWrite<u32, GPCLR1::Register>),
        (0x30 => @END),
    }
}

// generate GPFSELx scripts: https://pastebin.ubuntu.com/p/yYm5wVJnRV/
register_bitfields! {
    u32,

    GPFSEL0 [ // GPIO Function Select 1
        FSEL9 OFFSET(27) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL8 OFFSET(24) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL7 OFFSET(21) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL6 OFFSET(18) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL5 OFFSET(15) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL4 OFFSET(12) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL3 OFFSET(9)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL2 OFFSET(6)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL1 OFFSET(3)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL0 OFFSET(0)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
    ],
    GPFSEL1 [ // GPIO Function Select 1
        FSEL19 OFFSET(27) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL18 OFFSET(24) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL17 OFFSET(21) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL16 OFFSET(18) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL15 OFFSET(15) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL14 OFFSET(12) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL13 OFFSET(9)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL12 OFFSET(6)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL11 OFFSET(3)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL10 OFFSET(0)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
    ],
    GPFSEL2 [ // GPIO Function Select 2
        FSEL29 OFFSET(27) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL28 OFFSET(24) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL27 OFFSET(21) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL26 OFFSET(18) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL25 OFFSET(15) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL24 OFFSET(12) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL23 OFFSET(9)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL22 OFFSET(6)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL21 OFFSET(3)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL20 OFFSET(0)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
    ],
    GPFSEL3 [ // GPIO Function Select 3
        FSEL39 OFFSET(27) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL38 OFFSET(24) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL37 OFFSET(21) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL36 OFFSET(18) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL35 OFFSET(15) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL34 OFFSET(12) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL33 OFFSET(9)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL32 OFFSET(6)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL31 OFFSET(3)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL30 OFFSET(0)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
    ],
    GPFSEL4 [ // GPIO Function Select 4
        FSEL49 OFFSET(27) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL48 OFFSET(24) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL47 OFFSET(21) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL46 OFFSET(18) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL45 OFFSET(15) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL44 OFFSET(12) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL43 OFFSET(9)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL42 OFFSET(6)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL41 OFFSET(3)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL40 OFFSET(0)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
    ],
    GPFSEL5 [ // GPIO Function Select 5
        FSEL57 OFFSET(21) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL56 OFFSET(18) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL55 OFFSET(15) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL54 OFFSET(12) NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL53 OFFSET(9)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL52 OFFSET(6)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL51 OFFSET(3)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
        FSEL50 OFFSET(0)  NUMBITS(3) [ Input = 0b000, Output = 0b001, AltFunc0 = 0b100, AltFunc1 = 0b101, AltFunc2 = 0b110, AltFunc3 = 0b111, AltFunc4 = 0b011, AltFunc5 = 0b010, ],
    ],

    // ref: https://github.com/nihalpasham/rustBoot/blob/8437fd2a6ebf79d68a885da895e009fafccfccee/boards/hal/src/nxp/imx8mn/bsp/drivers/gpio.rs#L133
    GPSET0 [ // GPIO Pin Output set 0..31
        PIN OFFSET(0) NUMBITS(31) [],
    ],
    GPSET1 [ // GPIO Pin Output set 32..57
        PIN OFFSET(0) NUMBITS(31) [],
    ],
    GPCLR0 [ // GPIO Pin Output clear 0..31
        PIN OFFSET(0) NUMBITS(31) [],
    ],
    GPCLR1 [ // GPIO Pin Output clear 32..57
        PIN OFFSET(0) NUMBITS(31) [],
    ],
    // GPIO_PUP_PDN_REG https://pastebin.ubuntu.com/p/v6Hd4XKdMG/
}

pub struct GPIO {
    base: NonNull<GPIORegs>,
}

unsafe impl Send for GPIO {}
unsafe impl Sync for GPIO {}

impl GPIO {
    pub const unsafe fn new() -> Self {
        Self {
            base: NonNull::new(GPIO_REGS_BASE_ADDRESS).unwrap().cast(),
        }
    }

    const fn regs(&self) -> &GPIORegs {
        unsafe { self.base.as_ref() }
    }

    #[deprecated]
    pub fn init(&mut self) {
        trace!("init");
        todo!()
    }

    /// enable pins gpio_input
    pub fn enable_pin_input(&mut self, n: u8) {
        trace!("output {n}");
        let gpfsel_divisor = n / 10;

        match gpfsel_divisor {
            0 => self.regs().GPFSEL0.set(0b000 << (3 * n % 10)), // Input
            1 => self.regs().GPFSEL1.set(0b000 << (3 * n % 10)),
            2 => self.regs().GPFSEL2.set(0b000 << (3 * n % 10)),
            3 => self.regs().GPFSEL3.set(0b000 << (3 * n % 10)),
            4 => self.regs().GPFSEL4.set(0b000 << (3 * n % 10)),
            5 => self.regs().GPFSEL5.set(0b000 << (3 * n % 10)),
            _ => panic!("GPFSEL divisor overflow"),
        }
        trace!("output {n}");
    }

    /// enable pins gpio output
    pub fn enable_pin_output(&mut self, n: u8) {
        trace!("output {n}");
        let gpfsel_divisor = n / 10;

        match gpfsel_divisor {
            0 => self.regs().GPFSEL0.set(0b001 << (3 * n % 10)), // Output
            1 => self.regs().GPFSEL1.set(0b001 << (3 * n % 10)),
            2 => self.regs().GPFSEL2.set(0b001 << (3 * n % 10)),
            3 => self.regs().GPFSEL3.set(0b001 << (3 * n % 10)),
            4 => self.regs().GPFSEL4.set(0b001 << (3 * n % 10)),
            5 => self.regs().GPFSEL5.set(0b001 << (3 * n % 10)),
            _ => panic!("GPFSEL divisor overflow"),
        }
        trace!("output {n}");
    }

    // TODO Function names may need to be adjusted
    pub fn set_high(&mut self, n: u8) {
        trace!("set {n} hight");
        assert!(n < 57);
        // CHECK: Is it correct ?
        debug!("{} {}", n / 32, n % 32);
        match n / 32 {
            0 => self.regs().GPSET0.set(1 << (n % 32)), // Basically the same
            1 => self.regs().GPSET1.set(1 << (n % 32)),
            _ => panic!("Invalid value for gpio set_high"),
        }
        trace!("set {n} hight");
    }

    // TODO Function names may need to be adjusted
    pub fn set_low(&mut self, n: u8) {
        trace!("set {n} low");
        assert!(n < 57);
        // CHECK: Is it correct ?
        match n / 32 {
            0 => self.regs().GPCLR0.set(1 << (n % 32)),
            1 => self.regs().GPCLR1.set(1 << (n % 32)),
            _ => panic!("Invalid value for gpio set_high"),
        }
        trace!("set {n} low");
    }
}
