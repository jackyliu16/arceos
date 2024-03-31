#![no_std]
#![no_main]

#[cfg(feature = "axstd")]
use axstd::println;

extern crate bcm2711_hal as hal;

use crate::hal::bcm2711_regs::sys_timer::SysTimer;
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
use hal::serial::Serial;
use hal::time::Bps;
use log::debug;

#[no_mangle]
fn main() {
    let gpio = GPIO::new();
    let mut mbox = Mailbox::new(MBOX::new());
    let clocks = Clocks::freeze(&mut mbox).unwrap();
    let gp = gpio.split();

    // LED test
    let mut led = gp.p19.into_push_pull_output();
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

    let mac_addr = EthernetAddress::from(*get_mac_address(&mut mbox).mac_address());
    debug!("mac_addr: {mac_addr}");

    let eth_devices = Devices::new();

    let rx_descriptors = unsafe {
        static mut RX_DESC: Descriptors = arr![Descriptor::zero(); 256];
        &mut RX_DESC[..]
    };

    let tx_descriptors = unsafe {
        static mut TX_DESC: Descriptors = arr![Descriptor::zero(); 256];
        &mut TX_DESC[..]
    };

    debug!("AA");

    let sys_timer = SysTimer::new();
    let mut sys_counter = sys_timer.split().sys_counter;

    debug!("BB");

    let mut eth = Eth::new(
        eth_devices,
        &mut sys_counter,
        mac_addr,
        rx_descriptors,
        tx_descriptors,
    )
    .unwrap();

    debug!("Ethernet initialized");
    debug!("Waiting for link-up");

    loop {
        let status = eth.status().unwrap();
        if status.link_status {
            writeln!(serial, "Link is up").ok();
            writeln!(serial, "Speed: {}", status.speed).ok();
            writeln!(serial, "Full duplex: {}", status.full_duplex).ok();

            assert_ne!(status.speed, 0, "Speed is 0");
            assert_eq!(status.full_duplex, true, "Not full duplex");
            break;
        }

        sys_counter.delay_ms(100_u32);
        debug!(".");
    }

    debug!("Recv loop");

    let forged_pkt: [u8; 60] = [
        0x3C, 0xE1, 0xA1, 0x4E, 0x48, 0x5C, 0xDC, 0xA6, 0x32, 0x2D, 0xD7, 0x6C, 0x88, 0x74, 0xE2,
        0xE4, 0x36, 0x23, 0xFD, 0xEA, 0xCA, 0x87, 0x49, 0x5B, 0xD0, 0x20, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    let mut cnt = 0;
    loop {
        debug!("Sending forged pkt {} bytes", forged_pkt.len());
        eth.send(forged_pkt.len(), |buf| {
            buf.copy_from_slice(&forged_pkt);
        })
        .unwrap();
    }

    // println!("Hello, world!");
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
