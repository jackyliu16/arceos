use crate::{AllocError, AllocResult};
use core::ptr::NonNull;
use core::alloc::Layout;
use super::holes::HoleList;

/// A simple single linked list heap
pub struct Heap<const PAGE_SIZE: usize> {
    used: usize,
    /// the addrs of each pages block start
    holes: HoleList,
    /// the boundary of page allocator
    boundary: (usize, usize),
}

impl<const PAGE_SIZE: usize> Heap<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            used: 0,
            holes: HoleList::empty(),
            boundary: (0, 0),
        }
    }
    pub fn boundary(&self) -> (usize, usize) { self.boundary }
    /// init the heap with (start, end]
    pub fn init(&mut self, bottom: usize, size: usize) {
        log::debug!("init heap with ({bottom:x}, {size:x})");
        self.boundary = (bottom, bottom + size);
        self.holes = unsafe { HoleList::new(bottom as *mut u8, size) }
    }
    pub fn allocate(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        log::debug!("allocated ({:x} with {:x})", layout.size(), layout.align());
        match self.holes.allocate_first_fit(layout) {
            Ok((ptr, aligned_layout)) => {
                self.used += aligned_layout.size();
                Ok(ptr)
            },
            Err(e) => Err(AllocError::NoMemory),
        }
    } 
    pub fn bottom(&self) -> usize { self.holes.bottom as usize }
    pub fn top(&self) -> usize { self.holes.top as usize }
    pub fn size(&self) -> usize { 
        unsafe { 
            self.holes.top.offset_from(self.holes.bottom) as usize
        }
    }
    pub fn used(&self) -> usize { self.used }
    pub fn free(&self) -> usize { self.size() - self.used }
} 