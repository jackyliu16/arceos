#![no_std]
#![no_main]

#[cfg(feature = "axstd")]
use axstd::println;

extern crate bcm2711_hal as hal;

use embedded_hal::digital::v2::OutputPin;
use hal::gpio::GpioExt;
use hal::bcm2711_regs::gpio::GPIO;
use hal::bcm2711_regs::mbox::MBOX;
// use hal::bcm2711_regs::sys_timer::SysTimer;
use hal::bcm2711_regs::uart3::UART3;
use hal::serial::Serial;
use hal::clocks::Clocks;
use hal::mailbox::Mailbox;
use hal::prelude::*;
use hal::time::Bps;
use core::fmt::Write;
use log::debug;

#[no_mangle]
fn main() {
    debug!("A");
    let gpio = GPIO::new();
    debug!("A");
    let mut mbox = Mailbox::new(MBOX::new());
    debug!("A");
    let clocks = Clocks::freeze(&mut mbox).unwrap();
    debug!("A");
    let gp = gpio.split();

    debug!("A");
    // LED test
    let mut led = gp.p19.into_push_pull_output();
    debug!("A");
    // for _ in 0..10 {
    //     led.set_high();
    //     delay(1);
    //     led.set_low();
    //     delay(1);
    // }

    // Serial test
    let tx = gp.p4.into_alternate_af4();
    let rx = gp.p5.into_alternate_af4();
    let mut serial = Serial::uart3(UART3::new(), (tx, rx), Bps(115200), clocks);

    loop {
        write!(serial, "UART0 example").ok();
        serial.write_str("AAA");
        serial.write(23);
        serial.write(85);
        delay(1);
    }

    // println!("Hello, world!");
}

fn delay(seconds: u64) {
    for i in 1..seconds+1 {
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
