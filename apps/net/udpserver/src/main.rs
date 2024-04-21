#![no_std]
#![no_main]
// #![cfg_attr(feature = "axstd", no_std)]
// #![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
extern crate axstd as std;

use axstd::io;
use axstd::net::{ToSocketAddrs, UdpSocket};
use core::format_args;
use core::iter::Iterator;
use core::result::Result::{Err, Ok};

const LOCAL_IP: &str = "10.0.2.15";
const LOCAL_PORT: u16 = 5555;

const TARGET_IP: &str = "10.0.2.16";
const TARGET_PORT: u16 = 5555;

const PUNCH_RECEIVED: [u8; 16] = [
    0x70, 0x75, 0x6E, 0x63, 0x68, 0x20, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x64, 0x20, 0x0A,
];

fn receive_loop() -> io::Result<()> {
    let addr = (LOCAL_IP, LOCAL_PORT).to_socket_addrs()?.next().unwrap();
    let local_sock = UdpSocket::bind(addr)?;
    local_sock.set_nonblocking(true);
    // let addr = (LOCAL_IP, LOCAL_PORT).to_socket_addrs()?.next().unwrap();
    // let target_sock = UdpSocket::bind(addr)?;

    let mut sign_in_buf = [
        0x46, 0x69, 0x6E, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6E, 0x74, 0x20, 0x00, 0x00, 0x20,
        0x70, 0x75, 0x6E, 0x63, 0x68, 0x20,
    ];

    sign_in_buf[12] = 0x00;
    sign_in_buf[13] = 0x01;

    let mut buf = [0u8; 1024];

    loop {
        local_sock.send_to(&sign_in_buf[..], (TARGET_IP, TARGET_PORT));
        // match local_sock.recv_from(&mut buf) {
        //     Ok((size, addr)) => {
        //         println!("recv: {}Bytes from {}", size, addr);
        //         let mid = core::str::from_utf8(&buf).unwrap();
        //
        //         println!("{}", mid);
        //         if &PUNCH_RECEIVED[..] == &buf[..PUNCH_RECEIVED.len()] {
        //             break;
        //         }
        //         buf = [0u8; 1024];
        //     }
        //     Err(e) => (),
        // };
        // log::debug!("=======================================");
        // delay(4);
    }
    Ok(())
}

// #[cfg_attr(feature = "axstd", no_mangle)]
#[no_mangle]
fn main() {
    println!("Hello, simple udp client!");
    receive_loop().expect("test udp client failed");
}

fn delay(seconds: u64) {
    for i in 1..seconds + 1 {
        println!("{}", i);

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
