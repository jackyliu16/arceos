//! A simple Early Allocator
//! Base on the homeworks need, the implement are as follow
//! 1. Byte allocator will not auto deallocate, it will only deallocate when used bytes is 0
//! 2. Page Allocator will not deallocate
//! TODO: implement auto deallocate

use core::alloc::Layout;
use core::ptr::NonNull;
use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};
use crate::hole::HoleList;
use buddy_system_allocator::linked_list::LinkedList;

const ORDER: usize = 2;

pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    bytePointer: usize, 
    heap: Heap<32>,

    /// control boundary of this allocator
    boundary: (usize, usize)
    // TODO If should contains this variables ?
    /// the mem has been allocated
    allocated: usize,
    /// the mem control by this allocator 
    total: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            bytePointer: 0,
            heap: Heap::<32>::new(),
            boundary: (0, 0),
            allocated: 0,
            total: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        // Convert start & size to Layout for the need of holes
    }
    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Err(AllocError::NoMemory)
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        todo!()
    }
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        todo!()
    }
    fn total_bytes(&self) -> usize {
        todo!()
    }
    fn used_bytes(&self) -> usize {
        todo!()
    }
    fn available_bytes(&self) -> usize {
        todo!()
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        todo!()
    }
    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        todo!()
    }
    fn total_pages(&self) -> usize {
        todo!()
    }
    fn used_pages(&self) -> usize {
        todo!()
    }
    fn available_pages(&self) -> usize {
        todo!()
    }
}

