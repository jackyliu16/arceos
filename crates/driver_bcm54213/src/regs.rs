#![allow(dead_code, unused)]
use core::fmt;

use tock_registers::{
    interfaces::{ReadWriteable, Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};

register_structs! {
    Channel {
        (0x00 => CS: ReadWrite<u32, CS::Register>),
        (0x04 => CONBLK: ReadWrite<u32, CONBLK::Register>),
        (0x08 => TI: ReadWrite<u32, TI::Register>),
        (0x0c => S_AD: ReadWrite<u32, S_AD::Register>),
        (0x10 => D_AD: ReadWrite<u32, D_AD::Register>),
        (0x14 => TXFR_LEN: ReadWrite<u32, TXFR_LEN::Register>),
        (0x18 => STRIDE: ReadWrite<u32, STRIDE::Register>),
        (0x1c => NEXT_CONBLK: ReadWrite<u32, NEXT_CONBLK::Register>),
        (0x20 => DEBUG: ReadWrite<u32, DEBUG::Register>),
        (0x24 => @END),
    },
    // TODO CHECK
    LiteChannel {
        (0x00 => CS: ReadWrite<u32, CS::Register>),
        (0x04 => CONBLK: ReadWrite<u32, CONBLK::Register>),
        (0x08 => TI_LITE: ReadWrite<u32, TI_LITE::Register>),
        (0x0c => S_AD: ReadWrite<u32, S_AD::Register>),
        (0x10 => D_AD: ReadWrite<u32, D_AD::Register>),
        (0x14 => TXFR_LEN_LITE: ReadWrite<u32, TXFR_LEN_LITE::Register>),
        (0x18 => _reserved),
        (0x1c => NEXT_CONBLK: ReadWrite<u32, NEXT_CONBLK::Register>),
        (0x20 => DEBUG: ReadWrite<u32, DEBUG::Register>),
        (0x24 => @END),
    },
    // TODO CHECK
    DMA4Channel {
        (0x00 => CS: ReadWrite<u32, CS_DMA4::Register>),
        (0x04 => CB: ReadWrite<u32, CONBLK_DMA4::Register>),
        (0x08 => _reserved),
        (0x0c => DEBUG: ReadWrite<u32, DEBUG_DMA4::Register>),
        (0x10 => TI: ReadWrite<u32, TI_DMA4::Register>),
        (0x14 => SRC: ReadWrite<u32, SRC_DMA4::Register>),
        (0x18 => SRCI: ReadWrite<u32, SRCI_DMA4::Register>),
        (0x1c => DEST: ReadWrite<u32, DEST_DMA4::Register>),
        (0x20 => DESTI: ReadWrite<u32, DESTI_DMA4::Register>),
        (0x24 => LEN: ReadWrite<u32, LEN_DMA4::Register>),
        (0x28 => NEXT_CONBLK: ReadWrite<u32, NEXT_CONBLK_DMA4::Register>),
        (0x2c => DEBUG2: ReadWrite<u32, DEBUG2_DMA4::Register>),
        (0x30 => @END),
    },
    Bcm2711DMA {
        (0x000 => channel0: Channel),       // DMA Channel
        (0x024 => _reserved0),
        (0x100 => channel1: Channel),
        (0x124 => _reserved1),
        (0x200 => channel2: Channel),
        (0x224 => _reserved2),
        (0x300 => channel3: Channel),
        (0x324 => _reserved3),
        (0x400 => channel4: Channel),
        (0x424 => _reserved4),
        (0x500 => channel5: Channel),
        (0x524 => _reserved5),
        (0x600 => channel6: Channel),
        (0x624 => _reserved6),
        (0x700 => channel7: LiteChannel),   // Lite Channel
        (0x724 => _reserved7),
        (0x800 => channel8: LiteChannel),
        (0x824 => _reserved8),
        (0x900 => channel9: LiteChannel),
        (0x924 => _reserved9),
        (0xa00 => channel10: LiteChannel),
        (0xa24 => _reserved10),
        (0xb00 => channel11: DMA4Channel),  // DMA4 Channel
        (0xb30 => _reserved11),
        (0xc00 => channel12: DMA4Channel),
        (0xc30 => _reserved12),
        (0xd00 => channel13: DMA4Channel),
        (0xd30 => _reserved13),
        (0xe00 => channel14: DMA4Channel),
        (0xe30 => _reserved14),
        (0xfe0 => INT_STATUS: ReadWrite<u32, INT_STATUS::Register>),
        (0xfe4 => _reserved15),
        (0xff0 => ENABLE: ReadWrite<u32, GLOBAL_ENABLE::Register>),
        (0xff4 => @END),
    }
}

register_bitfields! {
    u32,

    // Control and Status registers
    CS [
        RESET OFFSET(31) NUMBITS(1),
        ABORT OFFSET(30) NUMBITS(1),
        DISDEBUG OFFSET(29) NUMBITS(1),
        WAIT_FOR_OUTS_TANDING_WRITES OFFSET(28) NUMBITS(1),
        PANIC_PRIORITY OFFSET(20) NUMBITS(3),
        PRIORITY OFFSET(16) NUMBITS(4),
        ERROR OFFSET(8) NUMBITS(1),
        WAITING_FOR_OUTSTANDING_WRITES OFFSET(6) NUMBITS(1),
        DREQ_STOPS_DMA OFFSET(5) NUMBITS(1),
        PAUSED OFFSET(4) NUMBITS(1),
        DREQ OFFSET(3) NUMBITS(1),
        INT OFFSET(2) NUMBITS(1),
        END OFFSET(1) NUMBITS(1),
        ACTIVE OFFSET(0) NUMBITS(1),
    ],

    // DMA4 Control and Status register
    CS_DMA4 [
        HALT OFFSET(31) NUMBITS(1),
        ABORT OFFSET(30) NUMBITS(1),
        DISDEBUG OFFSET(29) NUMBITS(1),
        WAIT_FOR_OUTS_TANDING_WRITES OFFSET(28) NUMBITS(1),
        OUTSTANDING_TRANSACTIONS OFFSET(25) NUMBITS(1),
        PANIC_PRIORITY OFFSET(20) NUMBITS(3),
        DMA_BUSY OFFSET(24) NUMBITS(1),
        PANIC_QOS OFFSET(20) NUMBITS(3),
        QOS OFFSET(16) NUMBITS(4),
        ERROR OFFSET(10) NUMBITS(1),
        WAITING_FOR_OUTSTANDING_WRITES OFFSET(7) NUMBITS(1),
        DREQ_STOPS_DMA OFFSET(6) NUMBITS(1),
        WR_PAUSED OFFSET(5) NUMBITS(1),
        RD_PAUSED OFFSET(4) NUMBITS(1),
        DREQ OFFSET(3) NUMBITS(1),
        INT OFFSET(2) NUMBITS(1),
        END OFFSET(1) NUMBITS(1),
        ACTIVE OFFSET(0) NUMBITS(1),
    ],

    // DMA Control Block Address register.
    CONBLK [
        SCB_ADDR OFFSET(0) NUMBITS(31),
    ],

    // DMA Next Control Block Address
    NEXT_CONBLK [
        ADDR OFFSET(0) NUMBITS(31),
    ],

    // DMA4 Control Block Address register
    CONBLK_DMA4 [
        ADDR OFFSET(0) NUMBITS(31),
    ],

    // DMA4 Next Control Block Address
    NEXT_CONBLK_DMA4 [
        ADDR OFFSET(0) NUMBITS(30),
    ],

    // DMA Transfer Information.
    TI [
        NO_WIDE_BURSTS OFFSET(26) NUMBITS(1),
        WAITS OFFSET(21) NUMBITS(5),
        PERMAP OFFSET(16) NUMBITS(4),
        BURST_LENGTH OFFSET(12) NUMBITS(3),
        SRC_IGNORE OFFSET(11) NUMBITS(1),
        SRC_DREQ OFFSET(10) NUMBITS(1),
        SRC_WIDTH OFFSET(9) NUMBITS(1),
        SRC_INC OFFSET(8) NUMBITS(1),
        DEST_IGNORE OFFSET(7) NUMBITS(1),
        DEST_DREQ OFFSET(6) NUMBITS(1),
        DEST_WIDTH OFFSET(5) NUMBITS(1),
        DEST_INC OFFSET(4) NUMBITS(1),
        WAIT_RESP OFFSET(3) NUMBITS(1),
        TDMODE OFFSET(1) NUMBITS(1),
        INTEN OFFSET(1) NUMBITS(1),
    ],

    // DMA Lite Transfer Information.
    TI_LITE [
        WAITS OFFSET(21) NUMBITS(5),
        PERMAP OFFSET(16) NUMBITS(4),
        BURST_LENGTH OFFSET(12) NUMBITS(3),
        SRC_DREQ OFFSET(10) NUMBITS(1),
        SRC_WIDTH OFFSET(9) NUMBITS(1),
        SRC_INC OFFSET(8) NUMBITS(1),
        DEST_DREQ OFFSET(6) NUMBITS(1),
        DEST_WIDTH OFFSET(5) NUMBITS(1),
        DEST_INC OFFSET(4) NUMBITS(1),
        WAIT_RESP OFFSET(3) NUMBITS(1),
        INTEN OFFSET(0) NUMBITS(1),
    ],

    // DMA4 Transfer Information
    TI_DMA4 [
        D_WAITS OFFSET(24) NUMBITS(8),
        S_WAITS OFFSET(16) NUMBITS(8),
        D_DREQ OFFSET(15) NUMBITS(1),
        S_DREQ OFFSET(14) NUMBITS(1),
        PERMAP OFFSET(9) NUMBITS(5),
        WAIT_RD_RESP OFFSET(3) NUMBITS(1),
        WAIT_RESP OFFSET(2) NUMBITS(1),
        TDMODE OFFSET(1) NUMBITS(1),
        INTEN OFFSET(1) NUMBITS(1),
    ],

    // DMA Source Address
    S_AD [
        S_ADDR OFFSET(0) NUMBITS(31),
    ],

    // DMA Destination Address
    D_AD [
        D_ADDR OFFSET(0) NUMBITS(31),
    ],

    // DMA Transfer Length.
    TXFR_LEN [
        YLENGTH OFFSET(16) NUMBITS(16),
        XLENGTH OFFSET(0) NUMBITS(16),
    ],

    //
    // DMA4 Transfer Length
    LEN_DMA4 [
        YLENGTH OFFSET(15) NUMBITS(15),
        XLENGTH OFFSET(0) NUMBITS(15),
    ],


    // DMA Lite Transfer Length
    TXFR_LEN_LITE [
        XLENGTH OFFSET(0) NUMBITS(16),
    ],

    // DMA 2D Stride
    STRIDE [
        D_STRIDE OFFSET(16) NUMBITS(16),
        S_STRIDE OFFSET(0)  NUMBITS(16),
    ],

    // DMA Debug register
    DEBUG [
        LITE OFFSET(28) NUMBITS(1),
        VERSION OFFSET(25) NUMBITS(3),
        DMA_STATE OFFSET(16) NUMBITS(8),
        DMA_ID OFFSET(8) NUMBITS(8),
        OUTSTANDING_WRITES OFFSET(4) NUMBITS(3),
        READ_ERROR OFFSET(2) NUMBITS(1),
        FIFO_ERROR OFFSET(1) NUMBITS(1),
        READ_LAST_NOT_SET_ERROR OFFSET(0) NUMBITS(1),
    ],

    // DMA Lite Debug register
    DEBUG_LITE [
        LITE OFFSET(28) NUMBITS(1),
        VERSION OFFSET(25) NUMBITS(3),
        DMA_STATE OFFSET(16) NUMBITS(8),
        DMA_ID OFFSET(8) NUMBITS(8),
        OUTSTANDING_WRITES OFFSET(4) NUMBITS(3),
        READ_ERROR OFFSET(2) NUMBITS(1),
        FIFO_ERROR OFFSET(1) NUMBITS(1),
        READ_LAST_NOT_SET_ERROR OFFSET(0) NUMBITS(1),
    ],

    // DMA4 Debug register
    DEBUG_DMA4 [
        VERSION OFFSET(28) NUMBITS(4),
        ID OFFSET(24) NUMBITS(3),
        RESET OFFSET(23) NUMBITS(1),
        W_STATE OFFSET(18) NUMBITS(4),
        R_STATE OFFSET(14) NUMBITS(4),
        DISABLE_CLK_GATE OFFSET(11) NUMBITS(1),
        ABORT_ON_ERROR OFFSET(10) NUMBITS(1),
        HALT_ON_ERROR OFFSET(9) NUMBITS(1),
        INT_ON_ERROR OFFSET(8) NUMBITS(1),
        READ_CB_ERROR OFFSET(3) NUMBITS(1),
        READ_ERROR OFFSET(2) NUMBITS(1),
        FIFO_ERROR OFFSET(1) NUMBITS(1),
        WRITE_ERROR OFFSET(0) NUMBITS(1),
    ],

    // DMA4 Debug2 register
    DEBUG2_DMA4 [
        OUTSTANDING_READS OFFSET(16) NUMBITS(8),
        OUTSTANDING_WRITES OFFSET(0) NUMBITS(8),
    ],

    // Lower 32 bits of the DMA4 Source Address
    SRC_DMA4 [
        ADDR OFFSET(0) NUMBITS(31),
    ],

    // Lower 32 bits of the DMA4 Destination Address
    DEST_DMA4 [
        ADDR OFFSET(0) NUMBITS(31),
    ],

    // DMA4 Source Information
    SRCI_DMA4 [
        STRIDE OFFSET(16) NUMBITS(16),
        IGNORE OFFSET(15) NUMBITS(1),
        SIZE OFFSET(13) NUMBITS(2),
        INC OFFSET(12) NUMBITS(1),
        BURST_LENGTH OFFSET(8) NUMBITS(4),
        ADDR OFFSET(0) NUMBITS(8),
    ],

    DESTI_DMA4 [
        STRIDE OFFSET(16) NUMBITS(16),
        IGNORE OFFSET(15) NUMBITS(1),
        SIZE OFFSET(13) NUMBITS(2),
        INC OFFSET(12) NUMBITS(1),
        BURST_LENGTH OFFSET(8) NUMBITS(4),
        ADDR OFFSET(0) NUMBITS(8),
    ],

    // Interrupt status of each DMA engine
    INT_STATUS [
        INT15 OFFSET(15) NUMBITS(1),
        INT14 OFFSET(14) NUMBITS(1),
        INT13 OFFSET(13) NUMBITS(1),
        INT12 OFFSET(12) NUMBITS(1),
        INT11 OFFSET(11) NUMBITS(1),
        INT10 OFFSET(10) NUMBITS(1),
        INT9 OFFSET(9) NUMBITS(1),
        INT8 OFFSET(8) NUMBITS(1),
        INT7 OFFSET(7) NUMBITS(1),
        INT6 OFFSET(6) NUMBITS(1),
        INT5 OFFSET(5) NUMBITS(1),
        INT4 OFFSET(4) NUMBITS(1),
        INT3 OFFSET(3) NUMBITS(1),
        INT2 OFFSET(2) NUMBITS(1),
        INT1 OFFSET(1) NUMBITS(1),
        INT0 OFFSET(0) NUMBITS(1),
    ],

    // Global enable bits for each channel.
    GLOBAL_ENABLE [
        PAGELITE OFFSET(28) NUMBITS(4),
        PAGE OFFSET(24) NUMBITS(3),
        EN14 OFFSET(14) NUMBITS(1),
        EN13 OFFSET(13) NUMBITS(1),
        EN12 OFFSET(12) NUMBITS(1),
        EN11 OFFSET(11) NUMBITS(1),
        EN10 OFFSET(10) NUMBITS(1),
        EN9 OFFSET(9) NUMBITS(1),
        EN8 OFFSET(8) NUMBITS(1),
        EN7 OFFSET(7) NUMBITS(1),
        EN6 OFFSET(6) NUMBITS(1),
        EN5 OFFSET(5) NUMBITS(1),
        EN4 OFFSET(4) NUMBITS(1),
        EN3 OFFSET(3) NUMBITS(1),
        EN2 OFFSET(2) NUMBITS(1),
        EN1 OFFSET(1) NUMBITS(1),
        EN0 OFFSET(0) NUMBITS(1),
    ]
}
