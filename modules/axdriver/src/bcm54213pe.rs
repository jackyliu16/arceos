// use axalloc::global_allocator;
// use axhal::mem::{phys_to_virt, virt_to_phys};
// use axhal::irq::{register_handler_common, IrqHandler};
use axhal::time::{busy_wait, current_time, Duration};

use driver_net::bcm54213pe::Bcm54213peHal;
pub struct Bcm54213peHalImpl;

impl Bcm54213peHal for Bcm54213peHalImpl {
    // fn register_irq(irq_num: usize, handler: IrqHandler) -> bool {
    //     // register_handler_common(irq_num, handler)
    //     false
    // }
    // fn current_time() -> usize {
    //     current_time().as_millis() as usize
    // }
    fn ndelay(n_times: usize) {
        busy_wait(Duration::from_nanos(n_times.try_into().unwrap()));
    }
    // fn phys_to_virt(pa: usize) -> usize {
    //     let va = phys_to_virt(pa.into()).as_usize();
    //     va
    // }
    // fn virt_to_phys(va: usize) -> usize {
    //     let pa = virt_to_phys(va.into()).as_usize();
    //     pa
    // }
    // fn dma_alloc_pages(pages: usize) -> (usize, usize) {
    //     let vaddr = if let Ok(vaddr) = global_allocator().alloc_pages(pages, 0x1000) {
    //         vaddr
    //     } else {
    //         panic!("RxRing alloc_pages failed");
    //     };
    //     let paddr = virt_to_phys(vaddr.into()).as_usize();
    //
    //     // info!("dma_alloc_pages vaddr:{:x} paddr:{:x}", vaddr, paddr);
    //     (vaddr, paddr)
    // }
    // fn dma_free_pages(vaddr: usize, pages: usize) {
    //     global_allocator().dealloc_pages(vaddr, pages);
    // }
}
