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
    let mut socket = UdpSocket::bind(addr)?;
    socket.set_nonblocking(true);
    println!("listen on: {}", socket.local_addr().unwrap());
    let mut buf = [0u8; 1024];
    loop {
        log::debug!("loop");
        // 1. try to recevice data from eth
        // TODO: 看上去这个东西是阻塞的
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                //      if send data -> download into fpm383c
                //      elif send complete message -> light
                //      else none
                // log::debug!("recv: {}Bytes from {}", size, addr);
                // let mid = core::str::from_utf8(&buf).unwrap();
                // log::debug!("{}", mid);
                // let mid = ["response_", mid].join("");
                // socket.send_to(mid.as_bytes(), addr)?;
                buf = [0u8; 1024];
            }
            Err(e) => {}
        }
        log::debug!("loop");

        for item in fpm383c::SLICE_OF_GREENFLASHING_MODE {
            serial.write(item);
        }

        // 2. try to send Match and CheckMatch to fpm383c
        // 3. collect match result
        //      if match send packet to eth for report punch card
        //      else red light
        delay(10);
    }
}

#[no_mangle]
fn main() {
    log::info!("hello, world");

    main_loop().expect("loop failure");

    // let forged_pkt: [u8; 60] = [
    //     0x3C, 0xE1, 0xA1, 0x4E, 0x48, 0x5C, 0xDC, 0xA6, 0x32, 0x2D, 0xD7, 0x6C, 0x88, 0x74, 0xE2,
    //     0xE4, 0x36, 0x23, 0xFD, 0xEA, 0xCA, 0x87, 0x49, 0x5B, 0xD0, 0x20, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // ];
    //
    // loop {
    //     log::debug!("====================================");
    //     for item in Search_Fingerprint_Match_Start {
    //         serial.write(item);
    //     }
    //     delay(10);
    //     serial.get_frame();
    //     // log::debug!("Match: {:?}", serial.get_frame());
    //
    //     for item in Search_Fingerprint_Match_Result {
    //         serial.write(item);
    //     }
    //
    //     if let Some(frame) = serial.get_frame() {
    //         assert!(frame.check_command(CmdType::CheckMatchFingerprint));
    //
    //         // NOTE: 对于 match 事件来说，没有报错并不意味着成功，只有当匹配结果选项 = 1，
    //         // 或者说出现了匹配 ID 才能说明匹配成功
    //
    //         // log::debug!("{frame:?}")
    //         let data = frame.get_all_users_data();
    //         log::debug!("data: {data:?}");
    //
    //         let data = frame.get_user_data(0, 2);
    //         if frame.get_user_data(0, 2).iter().any(|&x| x != 0) {
    //             for item in slice_of_greenflashing_mode {
    //                 serial.write(item);
    //             }
    //
    //             eth.send(forged_pkt.len(), |buf| {
    //                 buf.copy_from_slice(&forged_pkt);
    //             })
    //             .unwrap();
    //         } else {
    //             for item in slice_of_redflashing_mode {
    //                 serial.write(item);
    //             }
    //         }
    //         serial.get_frame();
    //
    //         // if frame.get_error_code(CmdType::CheckMatchFingerprint) == Packet::ErrorCode::Ok {
    //         //     for item in slice_of_greenflashing_mode { serial.write(item); }
    //         //     delay(10);
    //         //     log::trace!("Result: {:?}", frame);
    //         // } else {
    //         //     log::debug!("{:?}", frame);
    //         // }
    //     }
    // }
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
    pub static SLICE_OF_GREENFLASHING_MODE: [u8; 23] = [
        0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x0C, 0x81, 0x00, 0x00, 0x00, 0x00,
        0x02, 0x0F, 0x04, 0x01, 0x14, 0x14, 0x02, 0xC0,
    ];
    pub static SLICE_OF_REDFLASHING_MODE: [u8; 23] = [
        0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x0C, 0x81, 0x00, 0x00, 0x00, 0x00,
        0x02, 0x0F, 0x04, 0x02, 0x14, 0x14, 0x02, 0xBF,
    ];
    pub static SEARCH_FINGERPRINT_MATCH_START: [u8; 18] = [
        0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x07, 0x86, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x21, 0xDE,
    ];
    pub static SEARCH_FINGERPRINT_MATCH_RESULT: [u8; 18] = [
        0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x07, 0x86, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x22, 0xDD,
    ];
}
