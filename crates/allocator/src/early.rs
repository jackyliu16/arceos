
use core::alloc::Layout;
use core::ptr::NonNull;
use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator};
use crate::buddy::linked_list::LinkedList;

pub star

pub struct EarlyAllocator {
    /// the list contains all mem block available
    free_blocks_list: LinkedList,
    /// total mem block control by this allocator: protect from dealloc uncontroled memory block
    total_blocks_list: LinkedList,

    // TODO If should contains this variables ?
    /// the mem has been allocated
    allocated: usize,
    /// the mem control by this allocator 
    total: usize,
}

impl EarlyAllocator {
    pub const fn new() -> Self {
        todo!()
    }
}

impl EarlyAllocator for BuddyByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        todo!()
    }
    fn add_memory(&mut self, start: usize, size: usize) {
        todo!()
    }
}

impl BaseAllocator for EarlyAllocator {
    fn init(&mut self, start: usize, size: usize) {
        todo!()
    }
    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Err(AllocError::NoMemory)
    }
}

impl ByteAllocator for EarlyAllocator {
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

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator {
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

