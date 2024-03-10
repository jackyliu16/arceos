macro_rules! write_volatile_wrapper {
    // NOTE: Inverted for easy inspection
    ($val:expr, $reg:expr) => {
        write_volatile((($reg) as *mut usize), $val);
    };
}

macro_rules! read_volatile_wrapper {
    ($reg:expr) => {
        read_volatile(($reg as *const u32));
    };
}
// // Generate I/O inline functions
// #define GENET_IO_MACRO(name, offset)				\
// static inline u32 name##_readl(u32 off)				\
// {								\
// 	return read32 (ARM_BCM54213_BASE + offset + off);	\
// }								\
// static inline void name##_writel(u32 val, u32 off)		\
// {								\
// 	write32 (ARM_BCM54213_BASE + offset + off, val);	\
// }

use crate::consts::*;
macro_rules! genet_io {
    ("ext", $reg:expr) => {
        (ARM_BCM54213_BASE + GENET_EXT_OFF + $reg)
    };
    ("umac", $reg:expr) => {
        (ARM_BCM54213_BASE + GENET_UMAC_OFF + $reg)
    };
    ("sys", $reg:expr) => {
        (ARM_BCM54213_BASE + GENET_SYS_OFF + $reg)
    };
    ("intrl2_0", $reg:expr) => {
        (ARM_BCM54213_BASE + GENET_INTRL2_0_OFF + $reg)
    };
    ("intrl2_1", $reg:expr) => {
        ARM_BCM54213_BASE + GENET_INTRL2_1_OFF + $reg
    };
    ("hfb", $reg:expr) => {
        ARM_BCM54213_BASE + HFB_OFFSET + $reg
    };
    ("hfb_reg", $reg:expr) => {
        ARM_BCM54213_BASE + HFB_REG_OFFSET + $reg
    };
    ("rbuf", $reg:expr) => {
        ARM_BCM54213_BASE + GENET_RBUF_OFF + $reg
    };
}
