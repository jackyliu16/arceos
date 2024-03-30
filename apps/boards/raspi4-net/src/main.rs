#![no_std]
#![no_main]

#[cfg(feature = "axstd")]
use axstd::println;

extern crate bcm2711_hal as hal;

use embedded_hal::digital::v2::OutputPin;
use hal::bcm2711_regs::gpio::GPIO;
use hal::gpio::GpioExt;

#[no_mangle]
fn main() {
    let gpio = GPIO::new();
    let gp = gpio.split();

    // LED test

    let mut led = gp.p19.into_push_pull_output();

    // for _ in 0..10 {
    //     led.set_high();
    //     delay(1);
    //     led.set_low();
    //     delay(1);
    // }

    println!("Hello, world!");
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
