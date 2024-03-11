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

//--------------------------------------------------
// Personal
//--------------------------------------------------

// BCM2711 provide 16 DMA Channel (Channel 15 exclusively used by VPU)

const DMA_CHANNEL_BASE: usize = ARM_IO_BASE + 0x7000;
const DMA_CHANNEL_OFFSET: usize = 0x0100;

//--------------------------------------------------
// Full Copy Of circle consts
//--------------------------------------------------
pub const GENET_V5: usize = 5;

// HW params for GENET_V5
pub const TX_QUEUES: usize = 4;
pub const TX_BDS_PER_Q: usize = 32;
pub const RX_QUEUES: usize = 0;
pub const RX_BDS_PER_Q: usize = 0;
pub const HFB_FILTER_CNT: usize = 48;
pub const HFB_FILTER_SIZE: usize = 128;
pub const QTAG_MASK: usize = 0x3F;
pub const HFB_OFFSET: usize = 0x8000;
pub const HFB_REG_OFFSET: usize = 0xFC00;
pub const RDMA_OFFSET: usize = 0x2000;
pub const TDMA_OFFSET: usize = 0x4000;
pub const WORDS_PER_BD: usize = 3;

pub const RX_BUF_LENGTH: usize = 2048;

// DMA descriptors
pub const TOTAL_DESC: usize = 256;

pub const DMA_DESC_LENGTH_STATUS: usize = 0x00;
pub const DMA_DESC_ADDRESS_LO: usize = 0x04;
pub const DMA_DESC_ADDRESS_HI: usize = 0x08;
// pub const DMA_DESC_SIZE: usize = WORDS_PER_BD * std::mem::size_of::<u32>();

// queues
pub const GENET_Q0_PRIORITY: usize = 0;
pub const GENET_Q16_RX_BD_CNT: usize = TOTAL_DESC - RX_QUEUES * RX_BDS_PER_Q;
pub const GENET_Q16_TX_BD_CNT: usize = TOTAL_DESC - TX_QUEUES * TX_BDS_PER_Q;
pub const TX_RING_INDEX: usize = 1;

// pub const GENET_TDMA_REG_OFF: usize = TDMA_OFFSET + TOTAL_DESC * DMA_DESC_SIZE;
// pub const GENET_RDMA_REG_OFF: usize = RDMA_OFFSET + TOTAL_DESC * DMA_DESC_SIZE;

// DMA configuration
pub const DMA_MAX_BURST_LENGTH: usize = 8;
pub const DMA_FC_THRESH_HI: usize = TOTAL_DESC >> 4;
pub const DMA_FC_THRESH_LO: usize = 5;

// Ethernet defaults
pub const ETH_FCS_LEN: usize = 4;
pub const ETH_ZLEN: usize = 60;
pub const ENET_MAX_MTU_SIZE: usize = 1536;

// HW register offset and field definitions
pub const UMAC_HD_BKP_CTRL: usize = 0x004;
pub const HD_FC_EN: usize = 1 << 0;
pub const HD_FC_BKOFF_OK: usize = 1 << 1;
pub const IPG_CONFIG_RX_SHIFT: usize = 2;
pub const IPG_CONFIG_RX_MASK: usize = 0x1F;

pub const UMAC_CMD: usize = 0x008;
pub const CMD_TX_EN: usize = 1 << 0;
pub const CMD_RX_EN: usize = 1 << 1;
pub const UMAC_SPEED_10: usize = 0;
pub const UMAC_SPEED_100: usize = 1;
pub const UMAC_SPEED_1000: usize = 2;
pub const UMAC_SPEED_2500: usize = 3;
pub const CMD_SPEED_SHIFT: usize = 2;
pub const CMD_SPEED_MASK: usize = 3;
pub const CMD_PROMISC: usize = 1 << 4;
pub const CMD_PAD_EN: usize = 1 << 5;
pub const CMD_CRC_FWD: usize = 1 << 6;
pub const CMD_PAUSE_FWD: usize = 1 << 7;
pub const CMD_RX_PAUSE_IGNORE: usize = 1 << 8;
pub const CMD_TX_ADDR_INS: usize = 1 << 9;
pub const CMD_HD_EN: usize = 1 << 10;
pub const CMD_SW_RESET: usize = 1 << 13;
pub const CMD_LCL_LOOP_EN: usize = 1 << 15;
pub const CMD_AUTO_CONFIG: usize = 1 << 22;
pub const CMD_CNTL_FRM_EN: usize = 1 << 23;
pub const CMD_NO_LEN_CHK: usize = 1 << 24;
pub const CMD_RMT_LOOP_EN: usize = 1 << 25;
pub const CMD_PRBL_EN: usize = 1 << 27;
pub const CMD_TX_PAUSE_IGNORE: usize = 1 << 28;
pub const CMD_TX_RX_EN: usize = 1 << 29;
pub const CMD_RUNT_FILTER_DIS: usize = 1 << 30;

pub const UMAC_MAC0: usize = 0x00C;
pub const UMAC_MAC1: usize = 0x010;
pub const UMAC_MAX_FRAME_LEN: usize = 0x014;

pub const UMAC_MODE: usize = 0x44;
pub const MODE_LINK_STATUS: usize = 1 << 5;

pub const UMAC_EEE_CTRL: usize = 0x064;
pub const EN_LPI_RX_PAUSE: usize = 1 << 0;
pub const EN_LPI_TX_PFC: usize = 1 << 1;
pub const EN_LPI_TX_PAUSE: usize = 1 << 2;
pub const EEE_EN: usize = 1 << 3;
pub const RX_FIFO_CHECK: usize = 1 << 4;
pub const EEE_TX_CLK_DIS: usize = 1 << 5;
pub const DIS_EEE_10M: usize = 1 << 6;
pub const LP_IDLE_PREDICTION_MODE: usize = 1 << 7;

pub const UMAC_EEE_LPI_TIMER: usize = 0x068;
pub const UMAC_EEE_WAKE_TIMER: usize = 0x06C;
pub const UMAC_EEE_REF_COUNT: usize = 0x070;
pub const EEE_REFERENCE_COUNT_MASK: usize = 0xFFFF;

pub const UMAC_TX_FLUSH: usize = 0x334;

pub const UMAC_MIB_START: usize = 0x400;

pub const UMAC_MDIO_CMD: usize = 0x614;
pub const MDIO_START_BUSY: usize = 1 << 29;
pub const MDIO_READ_FAIL: usize = 1 << 28;
pub const MDIO_RD: usize = 2 << 26;
pub const MDIO_WR: usize = 1 << 26;
pub const MDIO_PMD_SHIFT: usize = 21;
pub const MDIO_PMD_MASK: usize = 0x1F;
pub const MDIO_REG_SHIFT: usize = 16;
pub const MDIO_REG_MASK: usize = 0x1F;

pub const UMAC_RBUF_OVFL_CNT_V1: usize = 0x61C;
pub const RBUF_OVFL_CNT_V2: usize = 0x80;
pub const RBUF_OVFL_CNT_V3PLUS: usize = 0x94;

pub const UMAC_MPD_CTRL: usize = 0x620;
pub const MPD_EN: usize = 1 << 0;
pub const MPD_PW_EN: usize = 1 << 27;
pub const MPD_MSEQ_LEN_SHIFT: usize = 16;
pub const MPD_MSEQ_LEN_MASK: usize = 0xFF;

pub const UMAC_MPD_PW_MS: usize = 0x624;
pub const UMAC_MPD_PW_LS: usize = 0x628;
pub const UMAC_RBUF_ERR_CNT_V1: usize = 0x634;
pub const RBUF_ERR_CNT_V2: usize = 0x84;
pub const RBUF_ERR_CNT_V3PLUS: usize = 0x98;
pub const UMAC_MDF_ERR_CNT: usize = 0x638;
pub const UMAC_MDF_CTRL: usize = 0x650;
pub const UMAC_MDF_ADDR: usize = 0x654;
pub const UMAC_MIB_CTRL: usize = 0x580;
pub const MIB_RESET_RX: usize = 1 << 0;
pub const MIB_RESET_RUNT: usize = 1 << 1;
pub const MIB_RESET_TX: usize = 1 << 2;

pub const RBUF_CTRL: usize = 0x00;
pub const RBUF_64B_EN: usize = 1 << 0;
pub const RBUF_ALIGN_2B: usize = 1 << 1;
pub const RBUF_BAD_DIS: usize = 1 << 2;

pub const RBUF_STATUS: usize = 0x0C;
pub const RBUF_STATUS_WOL: usize = 1 << 0;
pub const RBUF_STATUS_MPD_INTR_ACTIVE: usize = 1 << 1;
pub const RBUF_STATUS_ACPI_INTR_ACTIVE: usize = 1 << 2;

pub const RBUF_CHK_CTRL: usize = 0x14;
pub const RBUF_RXCHK_EN: usize = 1 << 0;
pub const RBUF_SKIP_FCS: usize = 1 << 4;

pub const RBUF_ENERGY_CTRL: usize = 0x9C;
pub const RBUF_EEE_EN: usize = 1 << 0;
pub const RBUF_PM_EN: usize = 1 << 1;

pub const RBUF_TBUF_SIZE_CTRL: usize = 0xB4;

pub const RBUF_HFB_CTRL_V1: usize = 0x38;
pub const RBUF_HFB_FILTER_EN_SHIFT: usize = 16;
pub const RBUF_HFB_FILTER_EN_MASK: usize = 0xFFFF0000;
pub const RBUF_HFB_EN: usize = 1 << 0;
pub const RBUF_HFB_256B: usize = 1 << 1;
pub const RBUF_ACPI_EN: usize = 1 << 2;

pub const RBUF_HFB_LEN_V1: usize = 0x3C;
pub const RBUF_FLTR_LEN_MASK: usize = 0xFF;
pub const RBUF_FLTR_LEN_SHIFT: usize = 8;

pub const TBUF_CTRL: usize = 0x00;
pub const TBUF_BP_MC: usize = 0x0C;
pub const TBUF_ENERGY_CTRL: usize = 0x14;
pub const TBUF_EEE_EN: usize = 1 << 0;
pub const TBUF_PM_EN: usize = 1 << 1;

pub const TBUF_CTRL_V1: usize = 0x80;
pub const TBUF_BP_MC_V1: usize = 0xA0;

pub const HFB_CTRL: usize = 0x00;
pub const HFB_FLT_ENABLE_V3PLUS: usize = 0x04;
pub const HFB_FLT_LEN_V2: usize = 0x04;
pub const HFB_FLT_LEN_V3PLUS: usize = 0x1C;

// uniMac intrl2 registers
pub const INTRL2_CPU_STAT: usize = 0x00;
pub const INTRL2_CPU_SET: usize = 0x04;
pub const INTRL2_CPU_CLEAR: usize = 0x08;
pub const INTRL2_CPU_MASK_STATUS: usize = 0x0C;
pub const INTRL2_CPU_MASK_SET: usize = 0x10;
pub const INTRL2_CPU_MASK_CLEAR: usize = 0x14;

// INTRL2 instance 0 definitions
pub const UMAC_IRQ_SCB: usize = 1 << 0;
pub const UMAC_IRQ_EPHY: usize = 1 << 1;
pub const UMAC_IRQ_PHY_DET_R: usize = 1 << 2;
pub const UMAC_IRQ_PHY_DET_F: usize = 1 << 3;
pub const UMAC_IRQ_LINK_UP: usize = 1 << 4;
pub const UMAC_IRQ_LINK_DOWN: usize = 1 << 5;
pub const UMAC_IRQ_LINK_EVENT: usize = UMAC_IRQ_LINK_UP | UMAC_IRQ_LINK_DOWN;
pub const UMAC_IRQ_UMAC: usize = 1 << 6;
pub const UMAC_IRQ_UMAC_TSV: usize = 1 << 7;
pub const UMAC_IRQ_TBUF_UNDERRUN: usize = 1 << 8;
pub const UMAC_IRQ_RBUF_OVERFLOW: usize = 1 << 9;
pub const UMAC_IRQ_HFB_SM: usize = 1 << 10;
pub const UMAC_IRQ_HFB_MM: usize = 1 << 11;
pub const UMAC_IRQ_MPD_R: usize = 1 << 12;
pub const UMAC_IRQ_RXDMA_MBDONE: usize = 1 << 13;
pub const UMAC_IRQ_RXDMA_PDONE: usize = 1 << 14;
pub const UMAC_IRQ_RXDMA_BDONE: usize = 1 << 15;
pub const UMAC_IRQ_RXDMA_DONE: usize = UMAC_IRQ_RXDMA_MBDONE;
pub const UMAC_IRQ_TXDMA_MBDONE: usize = 1 << 16;
pub const UMAC_IRQ_TXDMA_PDONE: usize = 1 << 17;
pub const UMAC_IRQ_TXDMA_BDONE: usize = 1 << 18;
pub const UMAC_IRQ_TXDMA_DONE: usize = UMAC_IRQ_TXDMA_MBDONE;

// Only valid for GENETv3+
pub const UMAC_IRQ_MDIO_DONE: usize = 1 << 23;
pub const UMAC_IRQ_MDIO_ERROR: usize = 1 << 24;

// INTRL2 instance 1 definitions
pub const UMAC_IRQ1_TX_INTR_MASK: usize = 0xFFFF;
pub const UMAC_IRQ1_RX_INTR_MASK: usize = 0xFFFF;
pub const UMAC_IRQ1_RX_INTR_SHIFT: usize = 16;

// Register block offsets
pub const GENET_SYS_OFF: usize = 0x0000;
pub const GENET_GR_BRIDGE_OFF: usize = 0x0040;
pub const GENET_EXT_OFF: usize = 0x0080;
pub const GENET_INTRL2_0_OFF: usize = 0x0200;
pub const GENET_INTRL2_1_OFF: usize = 0x0240;
pub const GENET_RBUF_OFF: usize = 0x0300;
pub const GENET_UMAC_OFF: usize = 0x0800;

// SYS block offsets and register definitions
pub const SYS_REV_CTRL: usize = 0x00;
pub const SYS_PORT_CTRL: usize = 0x04;
pub const PORT_MODE_INT_EPHY: usize = 0;
pub const PORT_MODE_INT_GPHY: usize = 1;
pub const PORT_MODE_EXT_EPHY: usize = 2;
pub const PORT_MODE_EXT_GPHY: usize = 3;
pub const PORT_MODE_EXT_RVMII_25: usize = 4 | (1 << 4);
pub const PORT_MODE_EXT_RVMII_50: usize = 4;
pub const LED_ACT_SOURCE_MAC: usize = 1 << 9;

pub const SYS_RBUF_FLUSH_CTRL: usize = 0x08;
pub const SYS_TBUF_FLUSH_CTRL: usize = 0x0C;
pub const RBUF_FLUSH_CTRL_V1: usize = 0x04;

// Ext block register offsets and definitions
pub const EXT_EXT_PWR_MGMT: usize = 0x00;
pub const EXT_PWR_DOWN_BIAS: usize = 1 << 0;
pub const EXT_PWR_DOWN_DLL: usize = 1 << 1;
pub const EXT_PWR_DOWN_PHY: usize = 1 << 2;
pub const EXT_PWR_DN_EN_LD: usize = 1 << 3;
pub const EXT_ENERGY_DET: usize = 1 << 4;
pub const EXT_IDDQ_FROM_PHY: usize = 1 << 5;
pub const EXT_IDDQ_GLBL_PWR: usize = 1 << 7;
pub const EXT_PHY_RESET: usize = 1 << 8;
pub const EXT_ENERGY_DET_MASK: usize = 1 << 12;
pub const EXT_PWR_DOWN_PHY_TX: usize = 1 << 16;
pub const EXT_PWR_DOWN_PHY_RX: usize = 1 << 17;
pub const EXT_PWR_DOWN_PHY_SD: usize = 1 << 18;
pub const EXT_PWR_DOWN_PHY_RD: usize = 1 << 19;
pub const EXT_PWR_DOWN_PHY_EN: usize = 1 << 20;

pub const EXT_RGMII_OOB_CTRL: usize = 0x0C;
pub const RGMII_LINK: usize = 1 << 4;
pub const OOB_DISABLE: usize = 1 << 5;
pub const RGMII_MODE_EN: usize = 1 << 6;
pub const ID_MODE_DIS: usize = 1 << 16;

pub const EXT_GPHY_CTRL: usize = 0x1C;
pub const EXT_CFG_IDDQ_BIAS: usize = 1 << 0;
pub const EXT_CFG_PWR_DOWN: usize = 1 << 1;
pub const EXT_CK25_DIS: usize = 1 << 4;
pub const EXT_GPHY_RESET: usize = 1 << 5;

// DMA rings size
pub const DMA_RING_SIZE: usize = 0x40;
// pub const DMA_RINGS_SIZE: usize = DMA_RING_SIZE * (GENET_DESC_INDEX + 1);

// DMA registers common definitions
pub const DMA_RW_POINTER_MASK: usize = 0x1FF;
pub const DMA_P_INDEX_DISCARD_CNT_MASK: usize = 0xFFFF;
pub const DMA_P_INDEX_DISCARD_CNT_SHIFT: usize = 16;
pub const DMA_BUFFER_DONE_CNT_MASK: usize = 0xFFFF;
pub const DMA_BUFFER_DONE_CNT_SHIFT: usize = 16;
pub const DMA_P_INDEX_MASK: usize = 0xFFFF;
pub const DMA_C_INDEX_MASK: usize = 0xFFFF;

// DMA ring size register
pub const DMA_RING_SIZE_MASK: usize = 0xFFFF;
pub const DMA_RING_SIZE_SHIFT: usize = 16;
pub const DMA_RING_BUFFER_SIZE_MASK: usize = 0xFFFF;

// DMA interrupt threshold register
pub const DMA_INTR_THRESHOLD_MASK: usize = 0x01FF;

// DMA XON/XOFF register
pub const DMA_XON_THREHOLD_MASK: usize = 0xFFFF;
pub const DMA_XOFF_THRESHOLD_MASK: usize = 0xFFFF;
pub const DMA_XOFF_THRESHOLD_SHIFT: usize = 16;

// DMA flow period register
pub const DMA_FLOW_PERIOD_MASK: usize = 0xFFFF;
pub const DMA_MAX_PKT_SIZE_MASK: usize = 0xFFFF;
pub const DMA_MAX_PKT_SIZE_SHIFT: usize = 16;

// DMA control register
pub const DMA_EN: usize = 1 << 0;
pub const DMA_RING_BUF_EN_SHIFT: usize = 0x01;
pub const DMA_RING_BUF_EN_MASK: usize = 0xFFFF;
pub const DMA_TSB_SWAP_EN: usize = 1 << 20;

// DMA status register
pub const DMA_DISABLED: usize = 1 << 0;
pub const DMA_DESC_RAM_INIT_BUSY: usize = 1 << 1;

// DMA SCB burst size register
pub const DMA_SCB_BURST_SIZE_MASK: usize = 0x1F;

// DMA activity vector register
pub const DMA_ACTIVITY_VECTOR_MASK: usize = 0x1FFFF;

// DMA backpressure mask register
pub const DMA_BACKPRESSURE_MASK: usize = 0x1FFFF;
pub const DMA_PFC_ENABLE: usize = 1 << 31;

// DMA backpressure status register
pub const DMA_BACKPRESSURE_STATUS_MASK: usize = 0x1FFFF;

// DMA override register
pub const DMA_LITTLE_ENDIAN_MODE: usize = 1 << 0;
pub const DMA_REGISTER_MODE: usize = 1 << 1;

// DMA timeout register
pub const DMA_TIMEOUT_MASK: usize = 0xFFFF;
pub const DMA_TIMEOUT_VAL: usize = 5000; // micro seconds

// TDMA rate limiting control register
pub const DMA_RATE_LIMIT_EN_MASK: usize = 0xFFFF;

// TDMA arbitration control register
pub const DMA_ARBITER_MODE_MASK: usize = 0x03;
pub const DMA_RING_BUF_PRIORITY_MASK: usize = 0x1F;
pub const DMA_RING_BUF_PRIORITY_SHIFT: usize = 5;

pub fn dma_prio_reg_index(q: usize) -> usize {
    q / 6
}

pub fn dma_prio_reg_shift(q: usize) -> usize {
    (q % 6) * DMA_RING_BUF_PRIORITY_SHIFT
}

pub const DMA_RATE_ADJ_MASK: usize = 0xFF;

// Tx/Rx Dma Descriptor common bits
pub const DMA_BUFLENGTH_MASK: usize = 0x0fff;
pub const DMA_BUFLENGTH_SHIFT: usize = 16;
pub const DMA_OWN: usize = 0x8000;
pub const DMA_EOP: usize = 0x4000;
pub const DMA_SOP: usize = 0x2000;
pub const DMA_WRAP: usize = 0x1000;

// Tx specific Dma descriptor bits
pub const DMA_TX_UNDERRUN: usize = 0x0200;
pub const DMA_TX_APPEND_CRC: usize = 0x0040;
pub const DMA_TX_OW_CRC: usize = 0x0020;
pub const DMA_TX_DO_CSUM: usize = 0x0010;
pub const DMA_TX_QTAG_SHIFT: usize = 7;

// Rx Specific Dma descriptor bits
pub const DMA_RX_CHK_V3PLUS: usize = 0x8000;
pub const DMA_RX_CHK_V12: usize = 0x1000;
pub const DMA_RX_BRDCAST: usize = 0x0040;
pub const DMA_RX_MULT: usize = 0x0020;
pub const DMA_RX_LG: usize = 0x0010;
pub const DMA_RX_NO: usize = 0x0008;
pub const DMA_RX_RXER: usize = 0x0004;
pub const DMA_RX_CRC_ERROR: usize = 0x0002;
pub const DMA_RX_OV: usize = 0x0001;
pub const DMA_RX_FI_MASK: usize = 0x001F;
pub const DMA_RX_FI_SHIFT: usize = 0x0007;
pub const DMA_DESC_ALLOC_MASK: usize = 0x00FF;

pub const DMA_ARBITER_RR: usize = 0x00;
pub const DMA_ARBITER_WRR: usize = 0x01;
pub const DMA_ARBITER_SP: usize = 0x02;

// --------------------------------------------------
// Ref on u-boot
// --------------------------------------------------

// pub const SYS_REV_CTRL: usize = 0x00;
//
// pub const SYS_PORT_CTRL: usize = 0x04;
// pub const PORT_MODE_EXT_GPHY: usize = 3;
//
// pub const GENET_SYS_OFF: usize = 0x0000;
// pub const SYS_RBUF_FLUSH_CTRL: usize = GENET_SYS_OFF + 0x08;
// pub const SYS_TBUF_FLUSH_CTRL: usize = GENET_SYS_OFF + 0x0c;
//
// pub const GENET_EXT_OFF: usize = 0x0080;
// pub const EXT_RGMII_OOB_CTRL: usize = GENET_EXT_OFF + 0x0c;
// pub const RGMII_LINK: usize = 1 << 4;
// pub const OOB_DISABLE: usize = 1 << 5;
// pub const RGMII_MODE_EN: usize = 1 << 6;
// pub const ID_MODE_DIS: usize = 1 << 16;
//
// pub const GENET_RBUF_OFF: usize = 0x0300;
// pub const RBUF_TBUF_SIZE_CTRL: usize = GENET_RBUF_OFF + 0xb4;
// pub const RBUF_CTRL: usize = GENET_RBUF_OFF;
// pub const RBUF_ALIGN_2B: usize = 1 << 1;
//
// pub const GENET_UMAC_OFF: usize = 0x0800;
// pub const UMAC_MIB_CTRL: usize = GENET_UMAC_OFF + 0x580;
// pub const UMAC_MAX_FRAME_LEN: usize = GENET_UMAC_OFF + 0x014;
// pub const UMAC_MAC0: usize = GENET_UMAC_OFF + 0x00c;
// pub const UMAC_MAC1: usize = GENET_UMAC_OFF + 0x010;
// pub const UMAC_CMD: usize = GENET_UMAC_OFF + 0x008;
// pub const MDIO_CMD: usize = GENET_UMAC_OFF + 0x614;
// pub const UMAC_TX_FLUSH: usize = GENET_UMAC_OFF + 0x334;
// pub const MDIO_START_BUSY: usize = 1 << 29;
// pub const MDIO_READ_FAIL: usize = 1 << 28;
// pub const MDIO_RD: usize = 2 << 26;
// pub const MDIO_WR: usize = 1 << 26;
// pub const MDIO_PMD_SHIFT: usize = 21;
// pub const MDIO_PMD_MASK: usize = 0x1f;
// pub const MDIO_REG_SHIFT: usize = 16;
// pub const MDIO_REG_MASK: usize = 0x1f;
//
// pub const CMD_TX_EN: usize = 1 << 0;
// pub const CMD_RX_EN: usize = 1 << 1;
// pub const UMAC_SPEED_10: usize = 0;
// pub const UMAC_SPEED_100: usize = 1;
// pub const UMAC_SPEED_1000: usize = 2;
// pub const UMAC_SPEED_2500: usize = 3;
// pub const CMD_SPEED_SHIFT: usize = 2;
// pub const CMD_SPEED_MASK: usize = 3;
// pub const CMD_SW_RESET: usize = 1 << 13;
// pub const CMD_LCL_LOOP_EN: usize = 1 << 15;
//
// pub const MIB_RESET_RX: usize = 1 << 0;
// pub const MIB_RESET_RUNT: usize = 1 << 1;
// pub const MIB_RESET_TX: usize = 1 << 2;

pub const TOTAL_DESCS: usize = 256;
pub const RX_DESCS: usize = TOTAL_DESCS;
pub const TX_DESCS: usize = TOTAL_DESCS;

pub const DEFAULT_Q: usize = 0x10;

pub const ENET_BRCM_TAG_LENGTH: usize = 6;
pub const ENET_PAD: usize = 8;
// pub const ENET_MAX_MTU_SIZE: usize =
//     ETH_DATA_LEN + ETH_HLEN + VLAN_HLEN + ENET_BRCM_TAG_LENGTH + ETH_FCS_LEN + ENET_PAD;
//
// pub const DMA_EN: usize = 1 << 0;
// pub const DMA_RING_BUF_EN_SHIFT: usize = 0x01;
// pub const DMA_RING_BUF_EN_MASK: usize = 0xffff;
// pub const DMA_BUFLENGTH_MASK: usize = 0x0fff;
// pub const DMA_BUFLENGTH_SHIFT: usize = 16;
// pub const DMA_RING_SIZE_SHIFT: usize = 16;
// pub const DMA_OWN: usize = 0x8000;
// pub const DMA_EOP: usize = 0x4000;
// pub const DMA_SOP: usize = 0x2000;
// pub const DMA_WRAP: usize = 0x1000;
// pub const DMA_MAX_BURST_LENGTH: usize = 0x8;
//
// pub const DMA_TX_UNDERRUN: usize = 0x0200;
// pub const DMA_TX_APPEND_CRC: usize = 0x0040;
// pub const DMA_TX_OW_CRC: usize = 0x0020;
// pub const DMA_TX_DO_CSUM: usize = 0x0010;
// pub const DMA_TX_QTAG_SHIFT: usize = 7;
//
// pub const DMA_RING_SIZE: usize = 0x40;
// pub const DMA_RINGS_SIZE: usize = DMA_RING_SIZE * (DEFAULT_Q + 1);
//
// pub const DMA_DESC_LENGTH_STATUS: usize = 0x00;
// pub const DMA_DESC_ADDRESS_LO: usize = 0x04;
// pub const DMA_DESC_ADDRESS_HI: usize = 0x08;
pub const DMA_DESC_SIZE: usize = 12;

pub const GENET_RX_OFF: usize = 0x2000;
pub const GENET_RDMA_REG_OFF: usize = GENET_RX_OFF + TOTAL_DESCS * DMA_DESC_SIZE;
pub const GENET_TX_OFF: usize = 0x4000;
pub const GENET_TDMA_REG_OFF: usize = GENET_TX_OFF + TOTAL_DESCS * DMA_DESC_SIZE;

// pub const DMA_FC_THRESH_HI: usize = RX_DESCS >> 4;
// pub const DMA_FC_THRESH_LO: usize = 5;
pub const DMA_FC_THRESH_VALUE: usize = (DMA_FC_THRESH_LO << 16) | DMA_FC_THRESH_HI;

// pub const DMA_XOFF_THRESHOLD_SHIFT: usize = 16;

pub const TDMA_RING_REG_BASE: usize = GENET_TDMA_REG_OFF + DEFAULT_Q * DMA_RING_SIZE;
pub const TDMA_READ_PTR: usize = TDMA_RING_REG_BASE + 0x00;
pub const TDMA_CONS_INDEX: usize = TDMA_RING_REG_BASE + 0x08;
pub const TDMA_PROD_INDEX: usize = TDMA_RING_REG_BASE + 0x0c;
pub const DMA_RING_BUF_SIZE: usize = 0x10;
pub const DMA_START_ADDR: usize = 0x14;
pub const DMA_END_ADDR: usize = 0x1c;
pub const DMA_MBUF_DONE_THRESH: usize = 0x24;
pub const TDMA_FLOW_PERIOD: usize = TDMA_RING_REG_BASE + 0x28;
pub const TDMA_WRITE_PTR: usize = TDMA_RING_REG_BASE + 0x2c;

pub const RDMA_RING_REG_BASE: usize = GENET_RDMA_REG_OFF + DEFAULT_Q * DMA_RING_SIZE;
pub const RDMA_WRITE_PTR: usize = RDMA_RING_REG_BASE + 0x00;
pub const RDMA_PROD_INDEX: usize = RDMA_RING_REG_BASE + 0x08;
pub const RDMA_CONS_INDEX: usize = RDMA_RING_REG_BASE + 0x0c;
pub const RDMA_XON_XOFF_THRESH: usize = RDMA_RING_REG_BASE + 0x28;
pub const RDMA_READ_PTR: usize = RDMA_RING_REG_BASE + 0x2c;

/* DMA rings size */
pub const DMA_RINGS_SIZE: usize = (DMA_RING_SIZE * (DEFAULT_Q + 1));

pub const TDMA_REG_BASE: usize = GENET_TDMA_REG_OFF + DMA_RINGS_SIZE;
pub const RDMA_REG_BASE: usize = GENET_RDMA_REG_OFF + DMA_RINGS_SIZE;
pub const DMA_RING_CFG: usize = 0x00;
pub const DMA_CTRL: usize = 0x04;
pub const DMA_SCB_BURST_SIZE: usize = 0x0c;

// pub const RX_BUF_LENGTH: usize = 2048;
pub const RX_TOTAL_BUFSIZE: usize = RX_BUF_LENGTH * RX_DESCS;
pub const RX_BUF_OFFSET: usize = 2;
