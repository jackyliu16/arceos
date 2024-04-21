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

const LOCAL_IP: &str = "0.0.0.0";
const LOCAL_PORT: u16 = 5555;

fn receive_loop() -> io::Result<()> {
    let addr = (LOCAL_IP, LOCAL_PORT).to_socket_addrs()?.next().unwrap();
    let socket = UdpSocket::bind(addr)?;
    println!("listen on: {}", socket.local_addr().unwrap());
    let mut buf = [0u8; 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                println!("recv: {}Bytes from {}", size, addr);
                let mid = core::str::from_utf8(&buf).unwrap();
                println!("{}", mid);
                let mid = ["response_", mid].join("");
                socket.send_to(mid.as_bytes(), addr)?;
                buf = [0u8; 1024];
            }
            Err(e) => return Err(e),
        };
    }
}

// #[cfg_attr(feature = "axstd", no_mangle)]
#[no_mangle]
fn main() {
    println!("Hello, simple udp client!");
    receive_loop().expect("test udp client failed");
}
