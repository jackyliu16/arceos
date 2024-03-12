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
use log::{debug, error, info, trace, warn};
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
    mac_addr: [u8; 6],
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
            mac_addr: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
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

    // TODO UNCHECK
    unsafe fn disable_dma(&self) {
        trace!("disable_dma");

        // clrbits_32(priv->mac_reg + TDMA_REG_BASE + DMA_CTRL, DMA_EN);
        // clrbits_32(priv->mac_reg + RDMA_REG_BASE + DMA_CTRL, DMA_EN);
        //
        // writel(1, priv->mac_reg + UMAC_TX_FLUSH);
        // udelay(10);
        // writel(0, priv->mac_reg + UMAC_TX_FLUSH);

        let reg = read_volatile_wrapper!(ARM_BCM54213_BASE + TDMA_REG_BASE + DMA_CTRL);
        reg & !(1 << DMA_EN);
        write_volatile_wrapper!(reg, ARM_BCM54213_BASE + TDMA_REG_BASE + DMA_CTRL);

        let reg = read_volatile_wrapper!(ARM_BCM54213_BASE + RDMA_REG_BASE + DMA_CTRL);
        reg & !(1 << DMA_EN);
        write_volatile_wrapper!(reg, ARM_BCM54213_BASE + RDMA_REG_BASE + DMA_CTRL);

        write_volatile_wrapper!(1, ARM_BCM54213_BASE + GENET_UMAC_OFF + UMAC_TX_FLUSH);
        A::udelay(10);
        write_volatile_wrapper!(0, ARM_BCM54213_BASE + GENET_UMAC_OFF + UMAC_TX_FLUSH);
    }

    // TODO UNCHECK
    unsafe fn enable_dma(&self) {
        trace!("enable_dma");
        // u32 dma_ctrl = (1 << (DEFAULT_Q + DMA_RING_BUF_EN_SHIFT)) | DMA_EN;
        // writel(dma_ctrl, priv->mac_reg + TDMA_REG_BASE + DMA_CTRL);
        // setbits_32(priv->mac_reg + RDMA_REG_BASE + DMA_CTRL, dma_ctrl);

        let dma_ctrl = (1 << (DEFAULT_Q + DMA_RING_BUF_EN_SHIFT)) | DMA_EN;
        write_volatile_wrapper!(
            dma_ctrl as u32,
            ARM_BCM54213_BASE + TDMA_REG_BASE + DMA_CTRL
        );

        let mut reg = read_volatile_wrapper!(ARM_BCM54213_BASE + RDMA_REG_BASE + DMA_CTRL);
        reg |= (dma_ctrl as u32);
        write_volatile_wrapper!(reg, ARM_BCM54213_BASE + RDMA_REG_BASE + DMA_CTRL);
    }

    // TODO uncheck
    unsafe fn rx_ring_init(&self) {
        trace!("rx_ring_init");
        write_volatile_wrapper!(
            DMA_MAX_BURST_LENGTH as u32,
            ARM_BCM54213_BASE + RDMA_REG_BASE + DMA_SCB_BURST_SIZE
        );
        trace!("rx_ring_init");

        write_volatile_wrapper!(0x0, ARM_BCM54213_BASE + RDMA_RING_REG_BASE + DMA_START_ADDR);
        write_volatile_wrapper!(0x0, ARM_BCM54213_BASE + RDMA_READ_PTR);
        write_volatile_wrapper!(0x0, ARM_BCM54213_BASE + RDMA_WRITE_PTR);
        write_volatile_wrapper!(
            (RX_DESCS * DMA_DESC_SIZE / 4 - 1) as u32,
            ARM_BCM54213_BASE + RDMA_RING_REG_BASE + DMA_END_ADDR
        );
        trace!("rx_ring_init");

        let c_index = read_volatile_wrapper!(ARM_BCM54213_BASE + RDMA_PROD_INDEX);
        write_volatile_wrapper!(c_index, ARM_BCM54213_BASE + RDMA_CONS_INDEX);
        write_volatile_wrapper!(
            ((RX_DESCS << DMA_RING_SIZE_SHIFT) | RX_BUF_LENGTH) as u32,
            ARM_BCM54213_BASE + RDMA_RING_REG_BASE + DMA_RING_BUF_SIZE
        );
        write_volatile_wrapper!(
            DMA_FC_THRESH_VALUE as u32,
            ARM_BCM54213_BASE + RDMA_XON_XOFF_THRESH
        );
        write_volatile_wrapper!(
            1 << DEFAULT_Q,
            ARM_BCM54213_BASE + RDMA_REG_BASE + DMA_RING_CFG
        );
        trace!("rx_ring_init");
        // 	priv->c_index = readl(priv->mac_reg + RDMA_PROD_INDEX);
        // 	writel(priv->c_index, priv->mac_reg + RDMA_CONS_INDEX);
        // 	priv->rx_index = priv->c_index;
        // 	priv->rx_index &= 0xFF;
    }

    // TODO UNCHECK
    unsafe fn rx_descs_init() {
        trace!("rx_descs_init");

        let len_stat = (RX_BUF_LENGTH << DMA_BUFLENGTH_SHIFT) | DMA_OWN;

        for i in 0..RX_DESCS {}
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
    // TODO UNCHECK
    unsafe fn tx_ring_init(&self) {
        trace!("tx_ring_init");
        write_volatile_wrapper!(
            DMA_MAX_BURST_LENGTH as u32,
            ARM_BCM54213_BASE + TDMA_REG_BASE + DMA_SCB_BURST_SIZE
        );

        write_volatile_wrapper!(0, ARM_BCM54213_BASE + TDMA_RING_REG_BASE + DMA_START_ADDR);
        write_volatile_wrapper!(0, ARM_BCM54213_BASE + TDMA_READ_PTR);
        write_volatile_wrapper!(0, ARM_BCM54213_BASE + TDMA_WRITE_PTR);
        write_volatile_wrapper!(
            (TX_DESCS * DMA_DESC_SIZE / 4 - 1) as u32,
            ARM_BCM54213_BASE + TDMA_RING_REG_BASE + DMA_END_ADDR
        );
        // 	priv->tx_index = readl(priv->mac_reg + TDMA_CONS_INDEX);
        // 	writel(priv->tx_index, priv->mac_reg + TDMA_PROD_INDEX);
        // 	priv->tx_index &= 0xFF;
        let tx_index = read_volatile_wrapper!(ARM_BCM54213_BASE + TDMA_CONS_INDEX);
        write_volatile_wrapper!(tx_index, ARM_BCM54213_BASE + TDMA_PROD_INDEX);
        write_volatile_wrapper!(
            0x1,
            ARM_BCM54213_BASE + TDMA_RING_REG_BASE + DMA_MBUF_DONE_THRESH
        );
        write_volatile_wrapper!(0x0, ARM_BCM54213_BASE + TDMA_FLOW_PERIOD);
        write_volatile_wrapper!(
            ((TX_DESCS << DMA_RING_SIZE_SHIFT) | RX_BUF_LENGTH) as u32,
            ARM_BCM54213_BASE + TDMA_RING_REG_BASE + DMA_RING_BUF_SIZE
        );
        write_volatile_wrapper!(
            1 << DEFAULT_Q,
            ARM_BCM54213_BASE + TDMA_REG_BASE + DMA_RING_CFG
        );
    }

    // TODO UNCHECK
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

    // TODO uncheck
    unsafe fn gmac_write_hwaddr(&self) {
        trace!("gmac_write_hwaddr");
        // NOTE: automatically set MAC address
        let reg = 0;
        write_volatile_wrapper!(reg, genet_io!("umac", UMAC_MAC0)); // High
        write_volatile_wrapper!(reg, genet_io!("umac", UMAC_MAC1)); // Low
    }

    unsafe fn hfd_init() {
        // this has no function, but to suppress warnings from clang compiler >>>
        // hfb_reg_readl(HFB_CTRL);
        // hfb_readl(0);
        // <<<
        write_volatile_wrapper!(0, genet_io!("hfb", HFB_CTRL));
        write_volatile_wrapper!(0, genet_io!("hfb", HFB_FLT_ENABLE_V3PLUS));
        write_volatile_wrapper!(0, genet_io!("hfb", HFB_FLT_ENABLE_V3PLUS + 4));

        for i in 0x70..0x8c { // DMA_INDEX2RING_0..DMA_INDEX2RING_7
             // TODO UNFINISH
             // The code write here should use rdma buffer
             // 	for (i = DMA_INDEX2RING_0; i <= DMA_INDEX2RING_7; i++)
             // 		rdma_writel(0, i);
             //
             // 	for (i = 0; i < (HFB_FILTER_CNT / 4); i++)
             // 		hfb_reg_writel(0, HFB_FLT_LEN_V3PLUS + i * sizeof(u32));
             //
             // 	for (i = 0; i < HFB_FILTER_CNT * HFB_FILTER_SIZE; i++)
             // 		hfb_writel(0, i * sizeof(u32));
             // }
        }
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

        // TODO should connect IRQ here
        // CInterruptSystem::Get ()->ConnectIRQ (ARM_IRQ_BCM54213_0, InterruptStub0, this);
        // CInterruptSystem::Get ()->ConnectIRQ (ARM_IRQ_BCM54213_1, InterruptStub1, this);

        // TODO mii_probe
        // ret = mii_probe();
        // if (ret)
        // {
        // 	CLogger::Get ()->Write (FromBcm54213, LogError, "Failed to connect to PHY (%d)", ret);
        //
        // 	CInterruptSystem::Get ()->DisconnectIRQ (ARM_IRQ_BCM54213_0);
        // 	CInterruptSystem::Get ()->DisconnectIRQ (ARM_IRQ_BCM54213_1);
        // 	m_bInterruptConnected = FALSE;
        //
        // 	return FALSE;
        // }
        //
        // netif_start();
        //
        // set_rx_mode ();
        //
        // AddNetDevice ();
    }

    // --------------------------------------------------
    // MII
    // --------------------------------------------------

    // TODO uncheck
    // Initialize link state variables that mii_setup() uses
    unsafe fn mii_probe(&self) {
        let m_old_link = -1;
        let m_old_speed = -1;
        let m_old_duplex = -1;
        let m_old_pause = -1;

        // probe PHY
        let ret = self.mdio_reset();
        warn!("mdio_reset Failure");
        self.mii_setup();

        self.mii_config();
    }

    // setup netdev link state when PHY link status change and
    // update UMAC and RGMII block when link up
    // TODO uncheck
    // TODO unfinish
    unsafe fn mii_setup(&self) {
        let status_changed = false;
        // bool status_changed = false;
        //
        // if (m_old_link != m_link) {
        //  status_changed = true;
        //  m_old_link = m_link;
        // }
        //
        // if (!m_link)
        //  return;
        //
        // // check speed/duplex/pause changes
        // if (m_old_speed != m_speed) {
        //  status_changed = true;
        //  m_old_speed = m_speed;
        // }
        //
        // if (m_old_duplex != m_duplex) {
        //  status_changed = true;
        //  m_old_duplex = m_duplex;
        // }
        //
        // if (m_old_pause != m_pause) {
        //  status_changed = true;
        //  m_old_pause = m_pause;
        // }
        //
        // // done if nothing has changed
        // if (!status_changed)
        //  return;
        //
        // // speed
        // u32 cmd_bits = 0;
        // if (m_speed == 1000)
        //  cmd_bits = UMAC_SPEED_1000;
        // else if (m_speed == 100)
        //  cmd_bits = UMAC_SPEED_100;
        // else
        //  cmd_bits = UMAC_SPEED_10;
        // cmd_bits <<= CMD_SPEED_SHIFT;
        //
        // // duplex
        // if (!m_duplex)
        //  cmd_bits |= CMD_HD_EN;
        //
        // // pause capability
        // if (!m_pause)
        //  cmd_bits |= CMD_RX_PAUSE_IGNORE | CMD_TX_PAUSE_IGNORE;
        //
        // // Program UMAC and RGMII block based on established
        // // link speed, duplex, and pause. The speed set in
        // // umac->cmd tell RGMII block which clock to use for
        // // transmit -- 25MHz(100Mbps) or 125MHz(1Gbps).
        // // Receive clock is provided by the PHY.
        // u32 reg = ext_readl(EXT_RGMII_OOB_CTRL);
        // reg &= ~OOB_DISABLE;
        // reg |= RGMII_LINK;
        // ext_writel(reg, EXT_RGMII_OOB_CTRL);
        //
        // reg = umac_readl(UMAC_CMD);
        // reg &= ~((CMD_SPEED_MASK << CMD_SPEED_SHIFT)
        //  | CMD_HD_EN
        //  | CMD_RX_PAUSE_IGNORE | CMD_TX_PAUSE_IGNORE);
        // reg |= cmd_bits;
        // umac_writel(reg, UMAC_CMD);
    }

    // TODO uncheck
    unsafe fn mii_config(&self) {
        // RGMII_NO_ID: TXC transitions at the same time as TXD
        //		(requires PCB or receiver-side delay)
        // RGMII:	Add 2ns delay on TXC (90 degree shift)
        //
        // ID is implicitly disabled for 100Mbps (RG)MII operation.
        let id_mode_dis = (1 << 16);
        write_volatile_wrapper!(PORT_MODE_EXT_GPHY as u32, genet_io!("sys", SYS_PORT_CTRL));
        // This is an external PHY (xMII), so we need to enable the RGMII
        // block for the interface to work

        let mut reg = read_volatile_wrapper!(genet_io!("ext", EXT_RGMII_OOB_CTRL));
        reg |= (RGMII_MODE_EN | id_mode_dis) as u32;
        write_volatile_wrapper!(reg, genet_io!("ext", EXT_RGMII_OOB_CTRL));
    }

    // --------------------------------------------------
    // MDIO
    // --------------------------------------------------

    // TODO uncheck
    unsafe fn mdio_reset(&self) -> isize {
        let ret = self.mdio_read(MII_BMSR);
        if ret < 0 {
            return ret;
        }
        0
    }

    // TODO unfinish
    // TODO uncheck
    unsafe fn mdio_read(&self, reg: usize) -> isize {
        let cmd = MDIO_RD | (PHY_ID << MDIO_PMD_SHIFT) | (reg << MDIO_PMD_SHIFT);
        write_volatile_wrapper!(cmd as u32, genet_io!("mdio", MDIO_CMD));

        self.mdio_start();
        self.mdio_wait();

        let cmd = read_volatile_wrapper!(genet_io!("mdio", MDIO_CMD));

        if cmd & (MDIO_READ_FAIL as u32) != 0 {
            warn!("mdio_read Failure");
            return -1;
        }

        (cmd & 0xFFFF) as isize
    }

    // TODO uncheck
    unsafe fn mdio_write(&self, reg: usize, val: usize) {
        let cmd = MDIO_WR | (PHY_ID << MDIO_PMD_SHIFT) | (reg << MDIO_REG_SHIFT) | (0xFFFF & val);
        write_volatile_wrapper!(cmd as u32, genet_io!("mdio", MDIO_CMD));

        self.mdio_start();

        self.mdio_wait();
    }

    // TODO uncheck
    unsafe fn mdio_start(&self) {
        trace!("mdio_start");
        let mut reg: u32 = read_volatile_wrapper!(genet_io!("mdio", MDIO_CMD));
        reg |= (MDIO_START_BUSY as u32);
        write_volatile_wrapper!(reg, genet_io!("mdio", MDIO_CMD));
    }

    // TODO uncheck
    unsafe fn mdio_wait(&self) {
        trace!("mdio_wait");
        let start_time = A::current_time();

        loop {
            if A::current_time() - start_time >= 1000 {
                break;
            }
            let reg = read_volatile_wrapper!(genet_io!("umac", UMAC_MDIO_CMD));
            if reg & (MDIO_START_BUSY as u32) != 0 {
                break;
            }
        }
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
