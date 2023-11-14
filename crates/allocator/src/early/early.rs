//! A simple Early Allocator
//! Base on the homeworks need, the implement are as follow
//! 1. Byte allocator will not auto deallocate, it will only deallocate when used bytes is 0
//! 2. Page Allocator will not deallocate
//! TODO: implement auto deallocate

use core::alloc::Layout;
use core::ptr::NonNull;
use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};
use bitmap_allocator::BitAlloc;
use crate::{align_up, align_down};

// Support max 1M * 4096 = 4GB memory.
type BitAllocUsed = bitmap_allocator::BitAlloc1M;

pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    /// the counter of how many times has calling for byte allocator
    used_bytes: usize, 
    /// if this pointer back to 0 -> deallocate all mem for byte allocator
    user: usize,

    /// control boundary of this allocator
    boundary: (usize, usize),

    base: usize,
    total_pages: usize,
    used_pages: usize,
    inner: BitAllocUsed,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            used_bytes: 0,
            user: 0,
            boundary: (0, 0),
            base: 0,
            used_pages: 0,
            total_pages: 0,
            inner: BitAllocUsed::DEFAULT,
        }
    }
    /// check if the allocator of bytes and pages will collision
    /// return true if will collision
    // #[deprecated]
    fn collision_detection(&self, layout: Layout) -> bool {
        // dbg!("collision_detection");
        // BC only page allocator will not be able to add more mem
        // self.used_bytes + self.boundary.0 + layout.size() > self.heap.boundary().0 
        // dbg!("{:x} {:x} {:x} {:x}", self.used_bytes, self.boundary.0, layout.size(), self.total_bytes());
        self.used_bytes + self.boundary.0 + layout.size() > self.base
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        // global
        self.boundary = (start, start + size);
        dbg!("Init Early Allocator at [{:x}, {:x})", start, start + size);

        // page allocate
        let start = start + size / 2;
        assert!(PAGE_SIZE.is_power_of_two());
        let end = align_down(start + size, PAGE_SIZE);
        let start = align_up(start, PAGE_SIZE);
        self.base = start;
        self.total_pages = (end - start) / PAGE_SIZE;
        self.inner.insert(0..self.total_pages);
        dbg!("Init Bitmap Allocator at [{:x}, {:x})", self.base, start + size / 2);
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
        axlog::trace!("allocate [{:x}, {:x})", 
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
        (self.boundary.1 - self.boundary.0 ) >> 1
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
        dbg!("alloc_pages");
        if align_pow2 % PAGE_SIZE != 0 {
            return Err(AllocError::InvalidParam);
        }
        let align_pow2 = align_pow2 / PAGE_SIZE;
        if !align_pow2.is_power_of_two() {
            return Err(AllocError::InvalidParam);
        }
        let align_log2 = align_pow2.trailing_zeros() as usize;
        match num_pages.cmp(&1) {
            core::cmp::Ordering::Equal => self.inner.alloc().map(|idx| idx * PAGE_SIZE + self.base),
            core::cmp::Ordering::Greater => self
                .inner
                .alloc_contiguous(num_pages, align_log2)
                .map(|idx| idx * PAGE_SIZE + self.base),
            _ => return Err(AllocError::InvalidParam),
        }
        .ok_or(AllocError::NoMemory)
        .inspect(|_| self.used_pages += num_pages)
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) { 
        self.used_pages -= num_pages;
        self.inner.dealloc((pos - self.base) / PAGE_SIZE)
    }
    fn total_pages(&self) -> usize {
        self.total_pages
    }
    fn used_pages(&self) -> usize {
        self.used_pages
    }
    fn available_pages(&self) -> usize {
        self.total_pages - self.used_pages
    }
}

#[deprecated]
fn find_rightest_matcher(start: usize, end: usize, align_pow2: usize, allocate_size: usize) -> AllocResult<usize> {
    dbg!("find_rightest_matcher");
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

