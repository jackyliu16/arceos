//! Definitions for PL011 UART.

#![no_std]
#![feature(const_option)]
#![feature(const_nonnull_new)]
#![feature(const_fmt_arguments_new)]

// cfg_if::cfg_if! {
//     // TODO: maybe we should refined, BC the register wasn't the same when we use this family?
//     if #[cfg(all(target_arch = "aarch64", platform_family = "aarch64-raspi"))] {
//         println!("A");
//         pub mod bcm2711;
//         pub use self::bcm2711::*;
//     // } else {
//     //     println!("B");
//     //     mod dummy;
//     //     pub use dummy::*;
//     } else {
//         println!("B");
//     }
// }
pub mod bcm2711;
pub use self::bcm2711::*;
