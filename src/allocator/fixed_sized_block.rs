use core::alloc::{GlobalAlloc, Layout};
use core::mem;
use crate::allocator::{BumpAllocator, Locked};

/// Available block sizes.
/// The block sizes must all be a power of 2 and must be in ascending order
/// because they are also used for block alignment (alignments must be
/// always powers of 2).
const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

struct ListNode {
    next: Option<&'static mut ListNode>,
}

pub struct FixedSizedBlockAllocator {
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
    fallback_allocator: Locked<BumpAllocator>,
}

impl FixedSizedBlockAllocator {
    /// Creates an empty FixedSizeBlockAllocator.
    pub const fn new() -> Self {
        const EMPTY: Option<&'static mut ListNode> = None;
        Self {
            list_heads: [EMPTY; BLOCK_SIZES.len()],
            fallback_allocator: Locked::new(BumpAllocator::new()),
        }
    }

    /// Initialize the memory with the given heap bounds.
    /// # Safety
    /// This function is unsafe because the caller must guarantee that the given
    /// heap bounds are valid and that the heap is unused. This method must be
    /// called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.fallback_allocator.lock().init(heap_start, heap_size);
    }

    unsafe fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
        self.fallback_allocator.alloc(layout)
    }

    unsafe fn fallback_dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        self.fallback_allocator.dealloc(ptr, layout)
    }
}

unsafe impl GlobalAlloc for Locked<FixedSizedBlockAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();
        match block_size_list_index(&layout) {
            Some(index) => match allocator.list_heads[index].take() {
                Some(node) => {
                    allocator.list_heads[index] = node.next.take();
                    node as *mut ListNode as *mut u8
                },
                None => {
                    // No block exists in the list yet.
                    // We must thus allocate a new one.
                    let block_size = BLOCK_SIZES[index];
                    // This only applies if block size is a power of 2.
                    let block_align = block_size;
                    let layout = Layout::from_size_align(block_size, block_align).unwrap();
                    allocator.fallback_alloc(layout)
                }
            }
            None => allocator.fallback_alloc(layout),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut allocator = self.lock();
        match block_size_list_index(&layout) {
            Some(index) => {
                let new_node = ListNode {
                    next: allocator.list_heads[index].take(),
                };

                // verify that block has size and alignment required for storing node
                assert!(mem::size_of::<ListNode>() <= BLOCK_SIZES[index]);
                assert!(mem::align_of::<ListNode>() <= BLOCK_SIZES[index]);

                let new_node_ptr = ptr as *mut ListNode;
                new_node_ptr.write(new_node);
                allocator.list_heads[index] = Some(&mut *new_node_ptr);
            }
            None => allocator.fallback_dealloc(ptr, layout)
        }
    }
}

/// Choose an appropriate block size for the given layout.
///
/// Returns an index into the `BLOCK_SIZES` array.
fn block_size_list_index(layout: &Layout) -> Option<usize> {
    let required_block_size = layout.size().max(layout.align());
    BLOCK_SIZES.iter().position(|&s| s >= required_block_size)
}