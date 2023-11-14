//! Single Linked List Heap
//! #Warn: deallocate unimplement

use super::linked_list::LinkedList;

/// A simple single linked list heap
pub struct Heap<const PAGE_SIZE: usize> {
    /// the addrs of each pages block start
    addrs: LinkedList,
    /// the boundary of page allocator
    boundary: (usize, usize),
}

impl<const PAGE_SIZE: usize> Heap<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            addrs: LinkedList::new(),
            boundary: (0, 0),
        }
    }
    pub fn boundary(&self) -> (usize, usize) { self.boundary }
    /// init the heap with (start, end]
    pub fn init(&mut self, start: usize, end: usize) {
        log::debug!("init heap with ({start:x}, {end:x})");
        self.boundary = (start, end)
    }
    pub fn push(&mut self, item: usize) {
        unsafe { self.addrs.push(item as *mut usize) };
    } 
    /// NOTE: haven't check
    pub fn count(&self) -> usize {
        not_implemented!("count");
        0
    }
    pub fn get_head(&self) -> usize {
        for addr in self.addrs.iter() {
            return addr as usize;
        }
        usize::MAX
    }
} 