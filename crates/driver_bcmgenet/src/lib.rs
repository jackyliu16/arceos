#![no_std]
#![allow(dead_code, unused)]

mod consts;
use consts::*;
// Firstly, All NIC device should implement NetDriverOps， which basically something like
// CNetDevice in Circle
// The crate will be implement in driver_net, not here. So will simple implement this crate base on
// convert of circle:bcm52413.cpp
