#![no_std]
#![no_main]

#[cfg(feature = "axstd")]
use axstd::println;

extern crate bcm2711_hal as hal;

use crate::hal::bcm2711_regs::sys_timer::SysTimer;
use embedded_hal::digital::v2::OutputPin;
use hal::bcm2711_regs::gpio::GPIO;
use hal::bcm2711_regs::mbox::MBOX;
use arm_gpio::GPIO as MyGPIO;
use hal::gpio::GpioExt;
// use hal::bcm2711_regs::sys_timer::SysTimer;
use crate::hal::eth::*;
use crate::hal::mailbox::{Channel, GetMacAddressRepr, Mailbox, RespMsg};
use arr_macro::arr;
use core::fmt::Write;
use hal::bcm2711_regs::uart3::UART3;
use hal::clocks::Clocks;
use hal::eth::EthernetAddress;
use hal::prelude::*;
use hal::serial::Serial;
use hal::time::Bps;
use log::debug;

#[no_mangle]
fn main() {
    let gpio = GPIO::new();
    let mut mbox = Mailbox::new(MBOX::new());
    let clocks = Clocks::freeze(&mut mbox).unwrap();
    let gp = gpio.split();

    // Serial test
    let tx = gp.p4.into_alternate_af4();
    let rx = gp.p5.into_alternate_af4();
    unsafe {
        let mut gpio: MyGPIO = MyGPIO::new();
        gpio.pup_pdn_control_reg(5, 0b01); // Pull Up
    }
    delay(3);
    let mut serial = Serial::uart3(UART3::new(), (tx, rx), Bps(57600), clocks);
    let slice_of_greenflashing_mode = [
        0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x0C, 0x81, 0x00, 0x00, 0x00, 0x00,
        0x02, 0x0F, 0x04, 0x01, 0x14, 0x14, 0x05, 0xBD,
    ];
    let slice_of_enrolled_fingerprint1 = [
        0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x08, 0x85, 
        0x00, 0x00, 0x00, 0x00, 0x01, 0x11, 
        0x01, 0xED,
    ];
    let slice_of_enrolled_fingerprint2 = [
        0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x08, 0x85, 
        0x00, 0x00, 0x00, 0x00, 0x01, 0x11, 
        0x02, 0xEC,
    ];
    let slice_of_enrolled_fingerprint3 = [
        0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x08, 0x85, 
        0x00, 0x00, 0x00, 0x00, 0x01, 0x11, 
        0x03, 0xEB,
    ];
    let slice_of_enrolled_fingerprint_check = [
        0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x07, 0x86, 
        0x00, 0x00, 0x00, 0x00, 0x01, 0x12, 
        0xED,
    ];
    debug!(":A:");
    
    for item in slice_of_enrolled_fingerprint1 { serial.write(item); }
    while let Some(data) = serial.get() {
        debug!("{data:x}");
    }
    debug!("AAA");
    while let Some(data) = serial.get() {
        debug!("{data:x}");
    }
    delay(10);
    for item in slice_of_enrolled_fingerprint_check { serial.write(item); }
    while let Some(data) = serial.get() {
        debug!("{data:x}");
    }
    loop {
        while let Some(data) = serial.get() {
            debug!("{data:x}");
        }
        debug!("EMPTY");
    }
    // loop {
    //     if let Some(data) = serial.get() {
    //         debug!("{data:x}");
    //     }
    // }
    // for item in slice_of_enrolled_fingerprint2 { serial.write(item); }
    // delay(10);
    // // for item in slice_of_enrolled_fingerprint3 { serial.write(item); }
    // // delay(10);
    //
    // delay(5);

}

fn delay(seconds: u64) {
    for i in 1..seconds + 1 {
        println!("{} ", i);

        fn fibonacci_recursive(n: u64) -> u64 {
            if n == 0 {
                return 0;
            }
            if n == 1 {
                return 1;
            }
            return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2);
        }

        fibonacci_recursive(34 + (i % 2));
    }
}
