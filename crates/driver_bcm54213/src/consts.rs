#![allow(unused)]
#![allow(non_snake_case)]
// --------------------------------------------------
// Ref on: circle/include/bcm2711.h
// --------------------------------------------------
pub const ARM_IO_BASE: usize = 0xFE000000;

pub const ARM_EMMC2_BASE: usize = (ARM_IO_BASE + 0x340000);

//
// Hardware Random Number Generator RNG200
//
pub const ARM_HW_RNG200_BASE: usize = (ARM_IO_BASE + 0x104000);

//
// Generic Interrupt Controller (GIC-400)
//
pub const ARM_GICD_BASE: usize = 0xFF841000;
pub const ARM_GICC_BASE: usize = 0xFF842000;
pub const ARM_GIC_END: usize = 0xFF847FFF;

//
// BCM54213PE Gigabit Ethernet Transceiver (external)
//
pub const ARM_BCM54213_BASE: usize = 0xFD580000;
pub const ARM_BCM54213_MDIO: usize = (ARM_BCM54213_BASE + 0x0E14);
pub const ARM_BCM54213_MDIO_END: usize = (ARM_BCM54213_BASE + 0x0E1B);
pub const ARM_BCM54213_END: usize = (ARM_BCM54213_BASE + 0xFFFF);

// Register block offsets
pub const GENET_SYS_OFF: usize = 0x0000;
pub const GENET_GR_BRIDGE_OFF: usize = 0x0040;
pub const GENET_EXT_OFF: usize = 0x0080;
pub const GENET_INTRL2_0_OFF: usize = 0x0200;
pub const GENET_INTRL2_1_OFF: usize = 0x0240;
pub const GENET_RBUF_OFF: usize = 0x0300;
pub const GENET_UMAC_OFF: usize = 0x0800;

//--------------------------------------------------
// Personal
//--------------------------------------------------

// BCM2711 provide 16 DMA Channel (Channel 15 exclusively used by VPU)

const DMA_CHANNEL_BASE: usize = ARM_IO_BASE + 0x7000;
const DMA_CHANNEL_OFFSET: usize = 0x0100;
