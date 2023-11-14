//! A simple Early Allocator
//! Base on the homeworks need, the implement are as follow
//! 1. Byte allocator will not auto deallocate, it will only deallocate when used bytes is 0
//! 2. Page Allocator will not deallocate
//! TODO: implement auto deallocate

#![allow(unused_imports, unused_variables, dead_code)]
use core::alloc::Layout;
use core::ptr::NonNull;
use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};
use super::heap::Heap;


pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    used_bytes: usize, 
    /// the counter of how many times has calling for byte allocator
    /// if this pointer back to 0 -> deallocate all mem for byte allocator
    user: usize,
    heap: Heap<32>,

    /// control boundary of this allocator
    boundary: (usize, usize),
    // TODO If should contains this variables ?
    /// the mem control by this allocator 
    total: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            used_bytes: 0,
            user: 0,
            heap: Heap::<32>::new(),
            boundary: (0, 0),
            total: 0,
        }
    }
    /// check if the allocator of bytes and pages will collision
    /// return true if will collision
    fn collision_detection(&self, layout: Layout) -> bool {
        // BC only page allocator will not be able to add more mem
        self.used_bytes + self.boundary.0 + layout.size() > self.heap.boundary().0 
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.boundary = (start, start + size);
        self.heap.init(start + size / 2, start + size);
        dbg!("init baseAllocator with [{:x}, {:x})", start + size / 2, start + size);
    }
    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        todo!()
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        if self.collision_detection(layout) {
            return Err(AllocError::NoMemory);
        }
        // allocate the memory of [boundary.0 + used_bytes, boundary.0 + used_bytes + layout.size())
        let p = unsafe { NonNull::new_unchecked((self.boundary.0 + self.used_bytes) as *mut u8) };
        self.used_bytes += layout.size();
        self.user += 1;
        dbg!("allocate [{:x}, {:x})", 
            self.boundary.0 + self.used_bytes,
            self.boundary.0 + self.used_bytes + layout.size(),
        );
        Ok(p)
    }
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        self.user -= 1;
        // if self.user == 0 then deallocate all memory
        if self.user == 0 {
            dbg!("clear mem of byte allocator");
            self.used_bytes = 0;
        }
    }
    fn total_bytes(&self) -> usize { 
        // all memory which haven't been allocate to heap will be in bytes allocate
        self.heap.boundary().0 - self.boundary.0 
    }
    fn used_bytes(&self) -> usize { self.used_bytes }
    fn available_bytes(&self) -> usize {
        self.total_bytes() - self.used_bytes()
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    /// allocate num_pages page with align in align_pow2
    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        // find first pages start addrs
        // BUG: haven't consider about request about more mem
        let (l, r) = (
            self.boundary.0,
            core::cmp::min(
                self.heap.get_head(),
                self.boundary.1,
            )
        );
        axlog::trace!("l: {l:x}");
        axlog::trace!("r: {r:x}");
        let addr = find_rightest_matcher(l, r, align_pow2, num_pages * PAGE_SIZE)?;
        axlog::trace!("alloc page addr: {addr:x} page num: {num_pages}");

        // NOTE: I have no ideas but it just not working.
        // let _ = (0..=num_pages)
        //     .map(|x| {
        //         dbg!("allocate {:x}", addr + x * PAGE_SIZE);
        //         self.heap.push(addr + x * PAGE_SIZE);
        //     });
        for i in 0..num_pages {
            axlog::trace!("{i}: allocate {:x}", addr + i * PAGE_SIZE);
            self.heap.push(addr + i * PAGE_SIZE);
        }
        dbg!("=======ONE FINISH==========");
        Ok(addr)
    }
    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) { 
        not_implemented!("dealloc_pages")
    }
    fn total_pages(&self) -> usize {
        not_implemented!("total_pages");
        0
    }
    fn used_pages(&self) -> usize {
        not_implemented!("used_pages");
        0
    }
    fn available_pages(&self) -> usize {
        not_implemented!("available_pages");
        0
    }
}

/// find the rightest addr which satisfy with page allocate and align_pows 
fn find_rightest_matcher(start: usize, end: usize, align_pow2: usize, allocate_size: usize) -> AllocResult<usize> {
    // match align (right first)
    let mut align_match = end + align_pow2 - end % align_pow2;
    // match page size
    let pages_match = end - allocate_size;

    while pages_match > start && align_match > start {
        axlog::trace!("align_match: {align_match:x}");
        axlog::trace!("pages_match: {pages_match:x}");

        if pages_match < align_match {
            // find left one
            align_match = pages_match - pages_match % align_pow2
        } else {
            return Ok(pages_match);
        }
    }
    Err(AllocError::NoMemory)
}
