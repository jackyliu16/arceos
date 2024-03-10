//! Dummy types used if no device of a certain category is selected.

#![allow(unused_imports)]
#![allow(dead_code)]

use super::prelude::*;
use log::trace;

use driver_net::bcm54213::Bcm54213HalTraits;
pub struct Bcm54213HalImpl;

impl Bcm54213HalTraits for Bcm54213HalImpl {
    fn dma_free_pages(vaddr: usize, pages: usize) {
        todo!()
    }

    fn dma_alloc_pages(pages: usize) -> (usize, usize) {
        todo!()
    }

    fn mdelay(m_times: usize) {
        todo!()
    }

    fn current_time() -> usize {
        todo!()
    }
}
