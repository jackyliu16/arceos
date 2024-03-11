#![no_std]
#![allow(dead_code, unused)]

extern crate alloc;

mod consts;
use consts::*;
mod netspeed;

#[macro_use]
mod macros;

use alloc::string::String;
use alloc::{fmt, format};
use core::marker::PhantomData;
use core::ptr::{read, read_volatile, write_volatile, NonNull};
use log::{debug, error, info, trace};
use netspeed::LinkSpeed;

pub trait Bcm54213HalTraits {
    fn phys_to_virt(pa: usize) -> usize {
        pa
    }
    fn virt_to_phys(va: usize) -> usize {
        va
    }
    fn dma_alloc_pages(pages: usize) -> (usize, usize);

    fn dma_free_pages(vaddr: usize, pages: usize);

    fn mdelay(m_times: usize);
    fn udelay(m_times: usize);

    fn current_time() -> usize;
}

pub struct Bcm54213NicDevice<A: Bcm54213HalTraits> {
    iobase_pa: usize,
    iobase_va: usize,
    link_speed: LinkSpeed,
    phantom: PhantomData<A>,
}

impl<A: Bcm54213HalTraits> Bcm54213NicDevice<A> {
    pub fn new(iobase_pa: usize) -> Self {
        debug!("Bcm54213NicDevice new");
        let iobase_va = A::phys_to_virt(iobase_pa);
        let mut nic = Bcm54213NicDevice::<A> {
            iobase_pa,
            iobase_va,
            link_speed: LinkSpeed::NetDeviceSpeedUnknown, // TODO
            phantom: PhantomData,
        };
        nic.init();
        nic
    }

    pub fn init(&mut self) -> bool {
        info!("Init Bcm54213NicDevice");

        // ref on circle:lib/bcm54213.cpp
        let mut reg = unsafe {
            read_volatile((ARM_BCM54213_BASE + GENET_SYS_OFF + SYS_REV_CTRL) as *const u32)
        };
        trace!("GENET HW version regs: {:0>32b}", reg);
        let major: u8 = (reg >> 24 & 0x0f) as u8;
        debug!("major: {:0>8b}", major);

        let major = match major {
            6 => 5,
            5 => 4,
            0 => 1,
            c => c,
        };
        if major != 5 {
            error!("GENET version mismatch, got: {major}, configured for: 5");
            return false;
        };

        assert!(((reg >> 16) & 0x0F) == 0); // minor version
        assert!((reg & 0xFFFF) == 0); // EPHY version

        unsafe {
            self.gmac_eth_start();
        }

        info!("Reach the end of Bcm54213NicDevice Init");
        true
    }

    unsafe fn disable_dma(&self) {
        trace!("disable_dma");
        clrbits_32(TDMA_REG_BASE, DMA_EN);
        trace!("A");
        clrbits_32(RDMA_REG_BASE, DMA_EN);
        trace!("A");

        trace!("A");
        write_volatile_wrapper!(1, genet_io!("umac", UMAC_TX_FLUSH));
        trace!("A");
        A::udelay(10);
        trace!("A");
        write_volatile_wrapper!(0, genet_io!("umac", UMAC_TX_FLUSH));
        trace!("A");
    }

    unsafe fn enable_dma(&self) {
        trace!("enable_dma");
        let reg = (1 << (DEFAULT_Q + DMA_RING_BUF_EN_SHIFT)) | DMA_EN;
        write_volatile_wrapper!(reg as u32, TDMA_REG_BASE + DMA_CTRL);
        setbits_32(RDMA_REG_BASE + DMA_CTRL, reg);
    }

    // TODO uncheck
    unsafe fn rx_ring_init(&self) {
        trace!("rx_ring_init");
        write_volatile_wrapper!(
            DMA_MAX_BURST_LENGTH as u32,
            RDMA_REG_BASE + DMA_SCB_BURST_SIZE
        );

        write_volatile_wrapper!(0x0, RDMA_RING_REG_BASE + DMA_START_ADDR);
        write_volatile_wrapper!(0x0, RDMA_READ_PTR);
        write_volatile_wrapper!(0x0, RDMA_WRITE_PTR);
        write_volatile_wrapper!(
            (RX_DESCS * DMA_DESC_SIZE / 4 - 1) as u32,
            RDMA_RING_REG_BASE + DMA_END_ADDR
        );

        let c_index = read_volatile_wrapper!(RDMA_PROD_INDEX);
        write_volatile_wrapper!(c_index, RDMA_CONS_INDEX);
        write_volatile_wrapper!(
            ((RX_DESCS << DMA_RING_SIZE_SHIFT) | RX_BUF_LENGTH) as u32,
            RDMA_RING_REG_BASE + DMA_RING_BUF_SIZE
        );
        write_volatile_wrapper!(DMA_FC_THRESH_VALUE as u32, RDMA_XON_XOFF_THRESH);
        write_volatile_wrapper!(1 << DEFAULT_Q, RDMA_REG_BASE + DMA_RING_CFG);
        // 	priv->c_index = readl(priv->mac_reg + RDMA_PROD_INDEX);
        // 	writel(priv->c_index, priv->mac_reg + RDMA_CONS_INDEX);
        // 	priv->rx_index = priv->c_index;
        // 	priv->rx_index &= 0xFF;
    }

    unsafe fn rx_descs_init() {
        trace!("rx_descs_init");
        todo!()
        //
        // 	void *desc_base = priv->rx_desc_base;

        // let len_stat = (RX_BUF_LENGTH << DMA_BUFLENGTH_SHIFT) | DMA_OWN;
        // for i in 0..RX_DESCS {
        //     write_volatile_wrapper!(rxbuff)
        // writel(lower_32_bits((uintptr_t)&rxbuffs[i * RX_BUF_LENGTH]),
        //        desc_base + i * DMA_DESC_SIZE + DMA_DESC_ADDRESS_LO);
        // writel(upper_32_bits((uintptr_t)&rxbuffs[i * RX_BUF_LENGTH]),
        //        desc_base + i * DMA_DESC_SIZE + DMA_DESC_ADDRESS_HI);
        // writel(len_stat,
        //        desc_base + i * DMA_DESC_SIZE + DMA_DESC_LENGTH_STATUS);
        // }
    }
    // static void rx_descs_init(struct bcmgenet_eth_priv *priv)
    // {
    // 	char *rxbuffs = &priv->rxbuffer[0];
    // 	u32 len_stat, i;
    // 	void *desc_base = priv->rx_desc_base;
    //
    // 	len_stat = (RX_BUF_LENGTH << DMA_BUFLENGTH_SHIFT) | DMA_OWN;
    //
    // 	for (i = 0; i < RX_DESCS; i++) {
    // 		writel(lower_32_bits((uintptr_t)&rxbuffs[i * RX_BUF_LENGTH]),
    // 		       desc_base + i * DMA_DESC_SIZE + DMA_DESC_ADDRESS_LO);
    // 		writel(upper_32_bits((uintptr_t)&rxbuffs[i * RX_BUF_LENGTH]),
    // 		       desc_base + i * DMA_DESC_SIZE + DMA_DESC_ADDRESS_HI);
    // 		writel(len_stat,
    // 		       desc_base + i * DMA_DESC_SIZE + DMA_DESC_LENGTH_STATUS);
    // 	}
    // }
    // TODO uncheck
    unsafe fn tx_ring_init(&self) {
        trace!("tx_ring_init");
        write_volatile_wrapper!(
            DMA_MAX_BURST_LENGTH as u32,
            TDMA_REG_BASE + DMA_SCB_BURST_SIZE
        );

        write_volatile_wrapper!(0, TDMA_RING_REG_BASE + DMA_START_ADDR);
        write_volatile_wrapper!(0, TDMA_READ_PTR);
        write_volatile_wrapper!(0, TDMA_WRITE_PTR);
        write_volatile_wrapper!(
            (TX_DESCS * DMA_DESC_SIZE / 4 - 1) as u32,
            TDMA_RING_REG_BASE + DMA_END_ADDR
        );
        // 	priv->tx_index = readl(priv->mac_reg + TDMA_CONS_INDEX);
        // 	writel(priv->tx_index, priv->mac_reg + TDMA_PROD_INDEX);
        // 	priv->tx_index &= 0xFF;
        let tx_index = read_volatile_wrapper!(TDMA_CONS_INDEX);
        write_volatile_wrapper!(tx_index, TDMA_PROD_INDEX);
        write_volatile_wrapper!(0x1, TDMA_RING_REG_BASE + DMA_MBUF_DONE_THRESH);
        write_volatile_wrapper!(0x0, TDMA_FLOW_PERIOD);
        write_volatile_wrapper!(
            ((TX_DESCS << DMA_RING_SIZE_SHIFT) | RX_BUF_LENGTH) as u32,
            TDMA_RING_REG_BASE + DMA_RING_BUF_SIZE
        );
        write_volatile_wrapper!(1 << DEFAULT_Q, TDMA_REG_BASE + DMA_RING_CFG);
    }

    // TODO uncheck
    unsafe fn umac_reset(&self) {
        trace!("umac_reset");
        let mut reg = read_volatile_wrapper!(genet_io!("sys", SYS_RBUF_FLUSH_CTRL));
        reg |= (1 << 1);
        write_volatile_wrapper!(reg, genet_io!("sys", SYS_RBUF_FLUSH_CTRL));

        A::udelay(10);

        reg &= !(1 << 1);
        write_volatile_wrapper!(reg, genet_io!("sys", SYS_RBUF_FLUSH_CTRL));
        A::udelay(10);
        write_volatile_wrapper!(0, genet_io!("umac", UMAC_CMD));
        write_volatile_wrapper!(
            (CMD_SW_RESET | CMD_LCL_LOOP_EN) as u32,
            genet_io!("umac", UMAC_CMD)
        );
        A::udelay(2);
        write_volatile_wrapper!(0, genet_io!("umac", UMAC_CMD));

        /* clear tx/rx counter */
        write_volatile_wrapper!(
            (MIB_RESET_RX | MIB_RESET_TX | MIB_RESET_RUNT) as u32,
            genet_io!("umac", UMAC_MIB_CTRL)
        );
        write_volatile_wrapper!(0, genet_io!("umac", UMAC_MIB_CTRL));
        // let reg = read_volatile_wrapper!(ARM_BCM54213_BASE + GENET_UMAC_OFF + UMAC_MAX_FRAME_LEN);
        // trace!("reg :{reg:0>32b}");
        // write_volatile(
        //     (ARM_BCM54213_BASE + GENET_UMAC_OFF + UMAC_MAX_FRAME_LEN) as *mut u32,
        //     ENET_MAX_MTU_SIZE as u32,
        // );
        // trace!("AAA");
        write_volatile_wrapper!(
            ENET_MAX_MTU_SIZE as u32,
            ARM_BCM54213_BASE + GENET_UMAC_OFF + UMAC_MAX_FRAME_LEN
        );

        /* init rx registers, enable ip header optimization */
        let mut reg = read_volatile_wrapper!(genet_io!("rbuf", RBUF_CTRL));
        reg |= (RBUF_ALIGN_2B as u32);
        write_volatile_wrapper!(reg, genet_io!("rbuf", RBUF_CTRL));

        // let reg = read_volatile_wrapper!(ARM_BCM54213_BASE + GENET_UMAC_OFF + RBUF_TBUF_SIZE_CTRL);
        // trace!("reg :{reg:0>32b}");

        write_volatile_wrapper!(1, genet_io!("rbuf", RBUF_TBUF_SIZE_CTRL));
    }

    // TODO unfinish
    unsafe fn gmac_write_hwaddr(&self) {
        trace!("gmac_write_hwaddr");
        // NOTE: automatically set MAC address
        let reg = 0;
        write_volatile_wrapper!(reg, genet_io!("umac", UMAC_MAC0)); // High
        write_volatile_wrapper!(reg, genet_io!("umac", UMAC_MAC1)); // Low
    }
    // --------------------------------------------------
    // GMAC ETH
    // --------------------------------------------------

    // TODO UNFINISH
    // TOOD UNCHECK
    unsafe fn gmac_eth_start(&self) {
        trace!("gmac_eth_start");
        let tx_desc_base = read_volatile_wrapper!(ARM_BCM54213_BASE + GENET_TX_OFF);
        let rx_desc_base = read_volatile_wrapper!(ARM_BCM54213_BASE + GENET_RX_OFF);

        trace!("1");
        self.umac_reset();
        trace!("2");
        self.gmac_write_hwaddr();
        /* Disable RX/TX DMA and flush TX queues */
        trace!("3");
        self.disable_dma();
        trace!("4");
        self.rx_ring_init();
        // 	rx_descs_init(priv);
        trace!("5");
        self.tx_ring_init();
        trace!("6");
        /* Enable RX/TX DMA */
        self.enable_dma();
        trace!("7");
        /* read PHY properties over the wire from generic PHY set-up */
        // ret = phy_startup(priv->phydev);
        // 	if (ret) {
        // 		printf("bcmgenet: PHY startup failed: %d\n", ret);
        // 		return ret;
        // 	}
        // 	/* Update MAC registers based on PHY property */
        // 	ret = bcmgenet_adjust_link(priv);
        // 	if (ret) {
        // 		printf("bcmgenet: adjust PHY link failed: %d\n", ret);
        // 		return ret;
        // 	}
        //
        /* Enable Rx/Tx */
        setbits_32(
            ARM_BCM54213_BASE + GENET_UMAC_OFF + UMAC_CMD,
            CMD_TX_EN | CMD_RX_EN,
        );
        trace!("8");
    }
}

pub trait CNetDevice {
    fn get_mac_address(&self) -> [u8; 6];
    fn get_link_speed(&self) -> String;
    fn get_net_device();
    fn is_send_frame_advisable() -> bool;
    fn is_link_up() -> bool;

    fn send_frame() -> bool;
    fn receive_frame() -> bool;

    fn update_phy() -> bool;
}

impl<A: Bcm54213HalTraits> CNetDevice for Bcm54213NicDevice<A> {
    fn get_mac_address(&self) -> [u8; 6] {
        trace!("get_mac_address");
        let mut ret: [u8; 6] = [0; 6];
        unsafe {
            // TODO CHANGE IT CORRECT
            let hi = read_volatile((self.iobase_va + 0) as *mut u32);
            let lo = read_volatile((self.iobase_va + 0) as *mut u32);
            ret[0] = (lo & 0xff) as u8;
            ret[1] = ((lo >> 8) & 0xff) as u8;
            ret[2] = ((lo >> 16) & 0xff) as u8;
            ret[3] = ((lo >> 24) & 0xff) as u8;
            ret[4] = (hi & 0xff) as u8;
            ret[5] = ((hi >> 8) & 0xff) as u8;
        }
        ret
    }
    fn get_link_speed(&self) -> String {
        trace!("get_link_speed");
        format!("{}", self.link_speed)
    }
    fn get_net_device() {
        trace!("get_net_device");
    }
    fn is_send_frame_advisable() -> bool {
        trace!("is_send_frame_advisable");
        true
    }
    fn is_link_up() -> bool {
        trace!("is_link_up");
        true
    }

    fn send_frame() -> bool {
        trace!("send_frame");
        true
    }
    fn receive_frame() -> bool {
        trace!("receive_frame");
        true
    }

    fn update_phy() -> bool {
        trace!("update_phy");
        false
    }
}

unsafe fn clrbits_32(addr: usize, clear: usize) {
    let mut reg = read_volatile(addr as *const u32);
    reg & (!(clear) as u32);
    write_volatile(addr as *mut u32, reg);
}

unsafe fn setbits_32(addr: usize, set: usize) {
    let mut reg = read_volatile(addr as *const u32);
    reg |= set as u32;
    write_volatile(addr as *mut u32, reg);
}

unsafe fn clrsetbits_32(addr: usize, clear: usize, set: usize) {
    let mut reg = read_volatile(addr as *const u32);
    reg & (!(clear) as u32);
    reg |= set as u32;
    write_volatile(addr as *mut u32, reg);
}
