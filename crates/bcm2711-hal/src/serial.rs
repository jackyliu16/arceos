//! Serial
//!
//! TODO - update this once bcm2711 docs are available
//!
//! There are two built-in UARTS, a PL011 (UART0)
//! and a mini UART (UART1).
//!
//! See the documentation:
//! https://www.raspberrypi.org/documentation/configuration/uart.md


use crate::clocks::Clocks;
use crate::gpio::{Alternate, Pin14, Pin15, Pin4, Pin5, AF0, AF4, AF5};
use crate::hal::prelude::*;
use crate::hal::serial;
use crate::time::Bps;
use bcm2711_regs::uart0::UART0;
use bcm2711_regs::uart1::UART1; // MINI
use bcm2711_regs::uart3::UART3;

use core::fmt;
use nb::block;
use void::Void;

pub trait Pins<UART> {}
pub trait PinTx<UART> {}
pub trait PinRx<UART> {}

impl<UART, TX, RX> Pins<UART> for (TX, RX)
where
    TX: PinTx<UART>,
    RX: PinRx<UART>,
{
}

impl PinTx<UART0> for Pin14<Alternate<AF0>> {}
impl PinRx<UART0> for Pin15<Alternate<AF0>> {}

impl PinTx<UART3> for Pin4<Alternate<AF4>> {}
impl PinRx<UART3> for Pin5<Alternate<AF4>> {}

/// Serial abstraction
pub struct Serial<UART, PINS> {
    uart: UART,
    pins: PINS,
}

impl<PINS> Serial<UART0, PINS> {
    pub fn uart0(mut uart: UART0, pins: PINS, baud_rate: Bps, clocks: Clocks) -> Self
    where
        PINS: Pins<UART0>,
    {
        use bcm2711_regs::uart0::*;
        let brr = if baud_rate.0 > (clocks.uart().0 / 16) {
            (clocks.uart().0 * 8) / baud_rate.0
        } else {
            (clocks.uart().0 * 4) / baud_rate.0
        };

        // Turn off UART0
        uart.cr
            .modify(Control::Enable::Clear + Control::TxEnable::Clear + Control::RxEnable::Clear);

        uart.icr.modify(IntClear::All::Clear);
        uart.ibrd
            .modify(IntegerBaudRateDivisor::Ibrd::Field::new(brr >> 6).unwrap());
        uart.fbrd
            .modify(FractionalBaudRateDivisor::Fbrd::Field::new(brr & 0x3F).unwrap());
        uart.lcrh.modify(LineControl::WordLength::EightBit); // 8N1
        uart.cr
            .modify(Control::Enable::Set + Control::TxEnable::Set + Control::RxEnable::Set);

        Serial { uart, pins }
    }

    pub fn free(self) -> (UART0, PINS) {
        (self.uart, self.pins)
    }
}

impl<PINS> serial::Write<u8> for Serial<UART0, PINS> {
    type Error = Void;

    fn flush(&mut self) -> nb::Result<(), Void> {
        use bcm2711_regs::uart0::Flag;
        if !self.uart.fr.is_set(Flag::TxFull::Read) {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Void> {
        use bcm2711_regs::uart0::{Data, Flag};
        if !self.uart.fr.is_set(Flag::TxFull::Read) {
            self.uart
                .dr
                .modify(Data::Data::Field::new(byte as _).unwrap());
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<PINS> fmt::Write for Serial<UART0, PINS> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            // Convert '\n' to '\r\n'
            if b as char == '\n' {
                block!(self.write('\r' as _)).ok();
            }
            block!(self.write(b)).ok();
        }
        Ok(())
    }
}

static header: [u8; 8] = [0xF1, 0x1F, 0xE2, 0x2E, 0xB6, 0x6B, 0xA8, 0x8A];
impl<PINS> Serial<UART3, PINS> {
    pub fn uart3(mut uart: UART3, pins: PINS, baud_rate: Bps, clocks: Clocks) -> Self
    where
        PINS: Pins<UART3>,
    {
        use bcm2711_regs::uart3::*;
        let brr = if baud_rate.0 > (clocks.uart().0 / 16) {
            (clocks.uart().0 * 8) / baud_rate.0
        } else {
            (clocks.uart().0 * 4) / baud_rate.0
        };

        // Turn off UART3
        uart.cr
            .modify(Control::Enable::Clear + Control::TxEnable::Clear + Control::RxEnable::Clear);

        uart.icr.modify(IntClear::All::Clear);
        uart.ibrd
            .modify(IntegerBaudRateDivisor::Ibrd::Field::new(brr >> 6).unwrap());
        uart.fbrd
            .modify(FractionalBaudRateDivisor::Fbrd::Field::new(brr & 0x3F).unwrap());
        uart.lcrh.modify(LineControl::WordLength::EightBit + LineControl::EnableFIFO::Enabled ); // 8N1
        uart.cr
            .modify(Control::Enable::Set + Control::TxEnable::Set + Control::RxEnable::Set);

        Serial { uart, pins }
    }

    pub fn free(self) -> (UART3, PINS) {
        (self.uart, self.pins)
    }
    pub fn get(&self) -> Option<u32> {
        use bcm2711_regs::uart3::{Data, Flag};
        if self.uart.fr.is_set(Flag::RxEmpty::Read) {
            // log::debug!("EMPTY");
            None
        } else {
            Some(self.uart.dr.get_field(Data::Data::Read).unwrap().val())
        }
    }
    pub fn get_frame(&self) -> Option<Packet::Frame> {
        // TODO: maybe circulation link list ?
        let mut buffer = [0_u8; 32]; // macher of header
        let mut cnt: u16 = 7;
        let mut len = 0;
        loop {
            if let Some(data) = self.get() {
                // if header haven't been match yet
                // log::trace!("{:?}", buffer);
                if &buffer[..8] != &header[..8] {
                    // log::trace!("{buffer:?}");
                    buffer.copy_within(1..8, 0);
                    // log::trace!("{buffer:?}");
                    buffer[8] = 0; // Clear the data left over from the previous operation
                    assert!(data <= u8::MAX as u32);
                    buffer[7] = data as u8;
                };
                // match header
                if &buffer[..8] == &header[..8] {
                    // log::trace!("match header");
                    buffer[cnt as usize] = data as u8;

                    // reach the end of frame
                    if cnt > 11 {
                        len = (buffer[8] as u16) << 8 | (buffer[9] as u16) << 0;
                    }
                    cnt += 1;
                };
            }

            if len != 0 && (cnt - 11) == len { // Frame header + len + parity
                log::trace!("{buffer:?}");
                // log::trace!("REACH THE END {len}");
                return Some(Packet::Frame::new(len, buffer[10], &buffer[11..cnt as usize]));
            }
        }
    }
}

impl<PINS> serial::Write<u8> for Serial<UART3, PINS> {
    type Error = Void;

    fn flush(&mut self) -> nb::Result<(), Void> {
        use bcm2711_regs::uart3::Flag;
        if !self.uart.fr.is_set(Flag::TxFull::Read) {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Void> {
        use bcm2711_regs::uart3::{Data, Flag};
        if !self.uart.fr.is_set(Flag::TxFull::Read) {
            self.uart
                .dr
                .modify(Data::Data::Field::new(byte as _).unwrap());
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<PINS> fmt::Write for Serial<UART3, PINS> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            // Convert '\n' to '\r\n'
            if b as char == '\n' {
                block!(self.write('\r' as _)).ok();
            }
            block!(self.write(b)).ok();
        }
        Ok(())
    }
}

impl<PINS> Serial<UART1, PINS> {
    pub fn uart1(mut uart: UART1, pins: PINS, baud_rate: Bps, clocks: Clocks) -> Self
    where
        PINS: Pins<UART1>,
    {
        use bcm2711_regs::uart1::*;
        // Mini UART uses 8-times oversampling
        // baudrate_reg = ((sys_clock / baudrate) / 8) - 1
        let brr = ((clocks.core().0 / baud_rate.0) / 8) - 1;

        uart.enable.modify(AuxEnable::MiniUartEnable::Set);
        uart.ier
            .modify(IntEnable::IntRx::Clear + IntEnable::IntTx::Clear);
        uart.cntl
            .modify(Control::RxEnable::Clear + Control::TxEnable::Clear);
        uart.lcr.modify(LineControl::DataSize::EightBit);
        uart.mcr.modify(ModemControl::Rts::Clear);
        uart.ier
            .modify(IntEnable::IntRx::Clear + IntEnable::IntTx::Clear);
        uart.iir.modify(IntIdentify::FifoClear::All);
        uart.baudrate
            .modify(Baudrate::Rate::Field::new(brr).unwrap());

        uart.cntl
            .modify(Control::RxEnable::Set + Control::TxEnable::Set);

        Serial { uart, pins }
    }

    pub fn free(self) -> (UART1, PINS) {
        (self.uart, self.pins)
    }
}

impl<PINS> serial::Read<u8> for Serial<UART1, PINS> {
    type Error = Void;

    fn read(&mut self) -> nb::Result<u8, Void> {
        use bcm2711_regs::uart1::{Data, LineStatus};
        if self.uart.lsr.is_set(LineStatus::DataReady::Read) {
            let mut data = self.uart.io.get_field(Data::Data::Read).unwrap().val() as u8;

            // convert carrige return to newline
            if data == '\r' as _ {
                data = '\n' as _;
            }

            Ok(data)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<PINS> serial::Write<u8> for Serial<UART1, PINS> {
    type Error = Void;

    fn flush(&mut self) -> nb::Result<(), Void> {
        use bcm2711_regs::uart1::LineStatus;
        if self.uart.lsr.is_set(LineStatus::TxEmpty::Read) {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Void> {
        use bcm2711_regs::uart1::{Data, LineStatus};
        if self.uart.lsr.is_set(LineStatus::TxEmpty::Read) {
            self.uart
                .io
                .modify(Data::Data::Field::new(byte as _).unwrap());
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<PINS> core::fmt::Write for Serial<UART1, PINS> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            // Convert '\n' to '\r\n'
            if b as char == '\n' {
                block!(self.write('\r' as _)).ok();
            }
            block!(self.write(b)).ok();
        }
        Ok(())
    }
}

#[allow(dead_code)]
pub mod Packet {
    const MAX_USER_LAYER_LENGTH: usize = 128 + 9;
    // TODO: refactor Debug trait 
    #[repr(packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Frame {
        // forhead: [u8; 6],
        length: u16,
        parity: u8,
        data: Data,
    }
    impl Frame {
        pub fn new(length: u16, parity: u8, data: &[u8]) -> Self {
            Self {
                length, parity, data: Data::parse(length, data)
            }
        }
        pub fn get_error_code(&self, cmd: CmdType) -> ErrorCode {
            let cmd_type = self.data.cmd_type;
            if cmd_type != cmd {
                ErrorCode::OtherError
            } else {
                self.data.error_code.clone()
            }
        }
    }

    #[repr(packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Data {
        pwd: u32,
        cmd_type: CmdType,
        error_code: ErrorCode,
        data: [u8; MAX_USER_LAYER_LENGTH], 
        checksum: u8,
    }
    impl Data {
        fn new() -> Self { Self { 
            pwd: 0, 
            cmd_type: CmdType::None, 
            error_code: ErrorCode::OtherError, 
            data: [0; MAX_USER_LAYER_LENGTH], 
            checksum:0 
        } }
        fn parse(length: u16, data: &[u8]) -> Self {
            log::debug!("user: {data:?}");
            Self {
                pwd: (data[0] as u32) << 24 | (data[1] as u32) << 16 | (data[2] as u32) << 8 | (data[3] as u32),
                cmd_type: ((data[4] as u16) << 8 | (data[5] as u16)).into(),
                error_code: data[9].into(),
                data: pad_slice(&data[9..(length as usize - 1)]),
                checksum: data[length as usize - 1]
            }
        }
    }
    fn pad_slice(slice: &[u8]) -> [u8; MAX_USER_LAYER_LENGTH] {
        let mut padded_slice = [0u8; MAX_USER_LAYER_LENGTH];
        for (i, elem) in slice.iter().chain(core::iter::repeat(&0)).take(MAX_USER_LAYER_LENGTH).enumerate() {
            padded_slice[i] = *elem;
            if i > MAX_USER_LAYER_LENGTH {
                log::error!("OVERFLOW BC data bit too long than we expect");
            }
        }
        padded_slice
    }

    #[repr(u16)]
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum CmdType {
        None = 0x0000,
        // Fingerprint
        FingerprintRegistration = 0x0111,
        CheckRegisterResult     = 0x0112,
        SaveFingerprint         = 0x0113,
        CheckSaveFingerprintResult = 0x0114,

        MatchFingerprint        = 0x0121,
        CheckMatchFingerprint   = 0x0122,

        // System
        LEDControl              = 0x020F,
    }
    impl From<CmdType> for u16 {
        fn from(cmd_type: CmdType) -> u16 {
            cmd_type as u16
        }
    }   
    impl From<u16> for CmdType {
        fn from(value: u16) -> CmdType {
            log::debug!("value: {value}");
            match value {
                0x0000 => CmdType::None,
                0x0111 => CmdType::FingerprintRegistration,
                0x0112 => CmdType::CheckRegisterResult,
                0x0113 => CmdType::SaveFingerprint,
                0x0114 => CmdType::CheckSaveFingerprintResult,

                0x0121 => CmdType::MatchFingerprint,
                0x0122 => CmdType::CheckMatchFingerprint,

                0x020F => CmdType::LEDControl,
                _ => panic!("Unknown CmdType value: {}", value),
            }
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum ErrorCode {
        Ok,
        UnknownCmd,
        CmdDataLenError,
        CmdDataError,
        CmdNotFinished,
        NoReqCmd,
        SysSoftError,
        HardwareError,
        NoFingerDetect,
        FingerExtractError,
        FingerMatchError,
        StorageIsFull,
        StorageWriteError,
        StorageReadError,
        UnqualifiedImageError,
        SameId,
        ImageLowCoverageError,
        CaptureLargeMove,
        CaptureNoMove,
        StorageRepeatFingerprint,
        CaptureImageFail,
        ForceQuit,
        NoneUpdate,
        InvalidFingerprintId,
        AdjustGainError,
        DataBufferOverflow,
        CurrentSensorSleep,
        PasswordError,
        ChecksumError,
        PinError,
        FlashIdError,
        ParameterError,
        ReadFtrError,
        FtrCrcErr,
        OtherError,
    }
    impl From<ErrorCode> for u8 {
        fn from(error_code: ErrorCode) -> u8 {
            error_code as u8
        }
    }   

    impl From<u8> for ErrorCode {
        fn from(code: u8) -> ErrorCode {
            match code {
                0x00 => ErrorCode::Ok,
                0x01 => ErrorCode::UnknownCmd,
                0x02 => ErrorCode::CmdDataLenError,
                0x03 => ErrorCode::CmdDataError,
                0x04 => ErrorCode::CmdNotFinished,
                0x05 => ErrorCode::NoReqCmd,
                0x06 => ErrorCode::SysSoftError,
                0x07 => ErrorCode::HardwareError,
                0x08 => ErrorCode::NoFingerDetect,
                0x09 => ErrorCode::FingerExtractError,
                0x0A => ErrorCode::FingerMatchError,
                0x0B => ErrorCode::StorageIsFull,
                0x0C => ErrorCode::StorageWriteError,
                0x0D => ErrorCode::StorageReadError,
                0x0E => ErrorCode::UnqualifiedImageError,
                0x0F => ErrorCode::SameId,
                0x10 => ErrorCode::ImageLowCoverageError,
                0x11 => ErrorCode::CaptureLargeMove,
                0x12 => ErrorCode::CaptureNoMove,
                0x13 => ErrorCode::StorageRepeatFingerprint,
                0x14 => ErrorCode::CaptureImageFail,
                0x15 => ErrorCode::ForceQuit,
                0x16 => ErrorCode::NoneUpdate,
                0x17 => ErrorCode::InvalidFingerprintId,
                0x18 => ErrorCode::AdjustGainError,
                0x19 => ErrorCode::DataBufferOverflow,
                0x1A => ErrorCode::CurrentSensorSleep,
                0x1B => ErrorCode::PasswordError,
                0x1C => ErrorCode::ChecksumError,
                0x1D => ErrorCode::PinError,
                0x20 => ErrorCode::FlashIdError,
                0x21 => ErrorCode::ParameterError,
                0x22 => ErrorCode::ReadFtrError,
                0x23 => ErrorCode::FtrCrcErr,
                0xFF => ErrorCode::OtherError,
                _ => ErrorCode::OtherError,
            }
        }
    }


}
