#![no_std]
#![no_main]

#[cfg(feature = "axstd")]
use axstd::println;

extern crate bcm2711_hal as hal;

use crate::hal::bcm2711_regs::sys_timer::SysTimer;
use crate::hal::eth::*;
use crate::hal::mailbox::{Channel, GetMacAddressRepr, Mailbox, RespMsg};
use arm_gpio::GPIO as MyGPIO;
use arr_macro::arr;
use axerrno::AxError;
use axstd::io;
use axstd::net::{IpAddr, SocketAddr, ToSocketAddrs, UdpSocket};
use axstd::println;
use core::fmt::Write;
use embedded_hal::digital::v2::OutputPin;
use hal::bcm2711_regs::gpio::GPIO;
use hal::bcm2711_regs::mbox::MBOX;
use hal::bcm2711_regs::uart3::UART3;
use hal::clocks::Clocks;
use hal::eth::EthernetAddress;
use hal::gpio::GpioExt;
use hal::prelude::*;
use hal::serial::Packet::*;
use hal::serial::{Packet, Serial};
use hal::time::Bps;
use log::{debug, info};

const LOCAL_IP: &str = "0.0.0.0";
const LOCAL_PORT: u16 = 5555;
// const LOCAL_IP: &str = "10.0.2.15";

const TARGET_IP: &str = "10.0.2.16";
const TARGET_PORT: u16 = 5555;

fn main_loop() -> io::Result<()> {
    let gpio = GPIO::new();
    let mut mbox = Mailbox::new(MBOX::new());
    let clocks = Clocks::freeze(&mut mbox).unwrap();
    let gp = gpio.split();

    let tx = gp.p4.into_alternate_af4();
    let rx = gp.p5.into_alternate_af4();
    unsafe {
        // Serial of Rx need to be PullUp
        let mut gpio: MyGPIO = MyGPIO::new();
        gpio.pup_pdn_control_reg(5, 0b01); // Pull Up
    }
    delay(3);
    let mut serial = Serial::uart3(UART3::new(), (tx, rx), Bps(57600), clocks);

    let addr = (LOCAL_IP, LOCAL_PORT).to_socket_addrs()?.next().unwrap();
    let mut local_socket = UdpSocket::bind(addr)?;
    // socket.set_nonblocking(true);
    println!("listen on: {}", local_socket.local_addr().unwrap());
    let mut buf = [0u8; 1024];
    loop {
        // NOTE: 如果需要确定与上一次match之间的时间差异
        // axhal::time:current_time().as_millis() as usize
        // let mut current = 0;
        log::debug!("====================================");
        // 1. try to receive data from eth(noblocking)
        // TODO: 还未完成
        // match socket.recv_from(&mut buf) {
        //     Ok((size, addr)) => buf = [0u8; 1024],
        //     Err(e) if e == AxError::WouldBlock => continue,
        //     Err(e) => {
        //         log::debug!("{e:?}");
        //     }
        // }
        log::debug!("AAA");
        // try to send match and check match to fpm383c
        for item in fpm383c::SEARCH_FINGERPRINT_MATCH_START {
            serial.write(item);
        }
        delay(10);
        serial.get_frame();
        // log::debug!("Match: {:?}", serial.get_frame());

        for item in fpm383c::CHECK_FINGERPRINT_MATCH_RESULT {
            serial.write(item);
        }

        if let Some(frame) = serial.get_frame() {
            assert!(frame.check_command(CmdType::CheckMatchFingerprint));

            // NOTE: 对于 match 事件来说，没有报错并不意味着成功，只有当匹配结果选项 = 1，
            // 或者说出现了匹配 ID 才能说明匹配成功

            // log::debug!("{frame:?}")
            let data = frame.get_all_users_data();
            log::debug!("data: {data:?}");

            let data = frame.get_user_data(0, 2);
            if frame.get_user_data(0, 2).iter().any(|&x| x != 0) {
                let mut sign_in_buf = [
                    0x46, 0x69, 0x6E, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6E, 0x74, 0x20, 0x00,
                    0x00, 0x20, 0x70, 0x75, 0x6E, 0x63, 0x68, 0x20,
                ];

                sign_in_buf[12] = frame.get_user_data(4, 5)[0];
                sign_in_buf[13] = frame.get_user_data(5, 6)[0];

                log::info!("{:?}", frame.get_user_data(4, 5));
                log::info!("{:?}", frame.get_user_data(5, 6));

                sign_in_buf[12] = 0x00;
                sign_in_buf[13] = 0x01;

                local_socket.send_to(&sign_in_buf[..], (TARGET_IP, TARGET_PORT));

                for item in fpm383c::SLICE_OF_GREENFLASHING_MODE {
                    serial.write(item);
                }
            } else {
                // TODO: should be remove
                for item in fpm383c::SLICE_OF_REDFLASHING_MODE {
                    serial.write(item);
                }
            }
            serial.get_frame();

            delay(10);
        }
    }
}

#[no_mangle]
fn main() {
    log::info!("hello, world");

    main_loop().expect("loop failure");
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

fn get_mac_address(mbox: &mut Mailbox) -> GetMacAddressRepr {
    let resp = mbox
        .call(Channel::Prop, &GetMacAddressRepr::default())
        .expect("MBox call()");

    if let RespMsg::GetMacAddress(repr) = resp {
        repr
    } else {
        panic!("Invalid response\n{:#?}", resp);
    }
}

mod fpm383c {
    pub const SLICE_OF_GREENFLASHING_MODE: [u8; 23] = [
        0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x0C, 0x81, 0x00, 0x00, 0x00, 0x00,
        0x02, 0x0F, 0x04, 0x01, 0x14, 0x14, 0x02, 0xC0,
    ];
    pub const SLICE_OF_REDFLASHING_MODE: [u8; 23] = [
        0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x0C, 0x81, 0x00, 0x00, 0x00, 0x00,
        0x02, 0x0F, 0x04, 0x02, 0x14, 0x14, 0x02, 0xBF,
    ];
    pub const SEARCH_FINGERPRINT_MATCH_START: [u8; 18] = [
        0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x07, 0x86, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x21, 0xDE,
    ];
    pub const CHECK_FINGERPRINT_MATCH_RESULT: [u8; 18] = [
        0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x07, 0x86, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x22, 0xDD,
    ];
    pub const PUNCH_RECEIVED: [u8; 16] = [
        0x70, 0x75, 0x6E, 0x63, 0x68, 0x20, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x64, 0x20,
        0x0A,
    ];
}
