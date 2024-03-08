#![allow(unused)]
// --------------------------------------------------
// Ref on: circle/include/bcm2711.h
// --------------------------------------------------
const ARM_IO_BASE: usize = 0xFE000000;

const ARM_EMMC2_BASE: usize = (ARM_IO_BASE + 0x340000);

//
// Hardware Random Number Generator RNG200
//
const ARM_HW_RNG200_BASE: usize = (ARM_IO_BASE + 0x104000);

//
// Generic Interrupt Controller (GIC-400)
//
const ARM_GICD_BASE: usize = 0xFF841000;
const ARM_GICC_BASE: usize = 0xFF842000;
const ARM_GIC_END: usize = 0xFF847FFF;

//
// BCM54213PE Gigabit Ethernet Transceiver (external)
//
const ARM_BCM54213_BASE: usize = 0xFD580000;
const ARM_BCM54213_MDIO: usize = (ARM_BCM54213_BASE + 0x0E14);
const ARM_BCM54213_MDIO_END: usize = (ARM_BCM54213_BASE + 0x0E1B);
const ARM_BCM54213_END: usize = (ARM_BCM54213_BASE + 0xFFFF);

//--------------------------------------------------
// Personal
//--------------------------------------------------

// BCM2711 provide 16 DMA Channel (Channel 15 exclusively used by VPU)

const DMA_CHANNEL_BASE: usize = 0x7E00_7000;
const DMA_CHANNEL_OFFSET: usize = 0x0100;
