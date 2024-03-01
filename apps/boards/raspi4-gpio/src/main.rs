#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

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
}
