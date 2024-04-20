#![no_std]
#![no_main]

#[cfg(feature = "axstd")]
use axstd::println;

extern crate bcm2711_hal as hal;

use crate::hal::bcm2711_regs::sys_timer::SysTimer;
use arm_gpio::GPIO as MyGPIO;
use embedded_hal::digital::v2::OutputPin;
use hal::bcm2711_regs::gpio::GPIO;
use hal::bcm2711_regs::mbox::MBOX;
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
use hal::serial::Packet::*;
use hal::serial::{Packet, Serial};
use hal::time::Bps;
use log::{debug, info};

#[no_mangle]
fn main() {
    log::info!("hello, world");
    // let gpio = GPIO::new();
    // let mut mbox = Mailbox::new(MBOX::new());
    // let clocks = Clocks::freeze(&mut mbox).unwrap();
    // let gp = gpio.split();
    //
    // // Serial test
    // let tx = gp.p4.into_alternate_af4();
    // let rx = gp.p5.into_alternate_af4();
    // unsafe {
    //     let mut gpio: MyGPIO = MyGPIO::new();
    //     gpio.pup_pdn_control_reg(5, 0b01); // Pull Up
    // }
    // delay(3);
    // let mut serial = Serial::uart3(UART3::new(), (tx, rx), Bps(57600), clocks);
    // debug!(":A:");
    //
    // let slice_of_greenflashing_mode = [
    //     0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x0C, 0x81, 0x00, 0x00, 0x00, 0x00,
    //     0x02, 0x0F, 0x04, 0x01, 0x14, 0x14, 0x02, 0xC0,
    // ];
    // let slice_of_redflashing_mode = [
    //     0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x0C, 0x81, 0x00, 0x00, 0x00, 0x00,
    //     0x02, 0x0F, 0x04, 0x02, 0x14, 0x14, 0x02, 0xBF,
    // ];
    //
    // let Search_Fingerprint_Match_Start = [
    //     0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x07, 0x86, 0x00, 0x00, 0x00, 0x00,
    //     0x01, 0x21, 0xDE,
    // ];
    // let Search_Fingerprint_Match_Result = [
    //     0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A, 0x00, 0x07, 0x86, 0x00, 0x00, 0x00, 0x00,
    //     0x01, 0x22, 0xDD,
    // ];
    // let mac_addr = EthernetAddress::from(*get_mac_address(&mut mbox).mac_address());
    // debug!("mac_addr: {mac_addr}");
    //
    // let eth_devices = Devices::new();
    // let rx_descriptors = unsafe {
    //     static mut RX_ESC: Descriptors = arr![Descriptor::zero(); 256];
    //     &mut RX_DESC[..]
    // };
    //
    // let tx_descriptors = unsafe {
    //     static mut TX_DESC: Descriptors = arr![Descriptor::zero(); 256];
    //     &mut TX_DESC[..]
    // };
    //
    // let sys_timer = SysTimer::new();
    // let mut sys_counter = sys_timer.split().sys_counter;
    // let mut eth = Eth::new(
    //     eth_devices,
    //     &mut sys_counter,
    //     mac_addr,
    //     rx_descriptors,
    //     tx_descriptors,
    // )
    // .unwrap();
    //
    // debug!("Ethernet initialized");
    // debug!("Waiting for link-up");
    //
    // loop {
    //     let status = eth.status().unwrap();
    //     if status.link_status {
    //         log::debug!("Link is up");
    //         log::debug!("Speed: {}", status.speed);
    //         log::debug!("Full duplex: {}", status.full_duplex);
    //         assert_ne!(status.speed, 0, "Speed is 0");
    //         assert_eq!(status.full_duplex, true, "Not full duplex");
    //         break;
    //     }
    //     sys_counter.delay_ms(100_u32);
    //     debug!(".");
    // }
    //
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
