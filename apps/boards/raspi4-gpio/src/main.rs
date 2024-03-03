#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

use arm_gpio::GPIO;
use arm_pl011::pl011::Pl011Uart;
use axstd::println;

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

#[no_mangle]
fn main() {
    println!("Hello, world!");

    let mut gpio: GPIO = GPIO::new();
    // gpio.enable_pin_output(20);
    gpio.enable_pin_output(21);
    println!("Hello, world!");

    // gpio.set_high(20);
    gpio.set_high(21);
    println!("Hello, world!");

    loop {
        // gpio.set_high(20);
        gpio.set_low(21);
        delay(3);
        gpio.set_high(21);
        // gpio.set_low(20);
        delay(3);
    }
}
