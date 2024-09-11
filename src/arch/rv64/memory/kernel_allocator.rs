use core::intrinsics::size_of;
use core::sync::atomic::{AtomicBool, Ordering};
use crate::arch::rv64::memory::page_allocator::*;
use crate::allocator::align_up;
use crate::arch::consts::get_page_size;
use crate::arch::paging_sv39::Table;
use crate::arch::rv64::memory::alloc_list::AllocList;
use crate::arch::rv64::memory::page_allocator;
use crate::arch::rv64::memory::page_allocator::zalloc;

static mut KMEM_HEAD: Option<*mut AllocList> = None;
/// The amount of memory (in pages) allocated by the kernel.
static mut KMEM_ALLOCATED: usize = 0;
static mut KMEM_PAGE_TABLE: Option<*mut Table> = None;
static mut IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn get_head() -> Option<*mut AllocList> {
    unsafe { KMEM_HEAD }
}

/// Wrapper around the kmem allocated variable to allow for safe access.
pub fn get_allocated_pages() -> usize {
    unsafe { KMEM_ALLOCATED }
}

/// Wrapper around the kmem page table to allow for safe access.
pub fn get_page_table() -> Option<*mut Table> {
    unsafe { KMEM_PAGE_TABLE }
}

/// Set the kernel memory page table.
pub fn set_page_table(table: *mut Table) {
    unsafe {
        KMEM_PAGE_TABLE = Some(table);
    }
}

/// Check if the kernel memory system is initialized.
pub fn is_initialized() -> bool {
    unsafe { IS_INITIALIZED.load(Ordering::Acquire) }
}

/// Initializes the kernel memory system.
/// This is only to be used by the kernel
/// and should not be called by the user.
/// Users should use [`page_allocator::alloc`]
/// or [`dealloc`] instead.
pub fn init() {
    assert!(!is_initialized(), "kernel memory already initialized");
    assert!(page_allocator::is_initialized(), "page allocator not initialized");

    unsafe {
        KMEM_ALLOCATED = 512;
        let alloc_list = zalloc(KMEM_ALLOCATED).expect("out of memory") as *mut AllocList;
        KMEM_HEAD = Some(alloc_list);
        (*KMEM_HEAD.unwrap()).set_free();
        (*KMEM_HEAD.unwrap()).set_size(KMEM_ALLOCATED * get_page_size());

        let page_table = zalloc(1).expect("out of memory") as *mut Table;
        KMEM_PAGE_TABLE = Some(page_table);
        IS_INITIALIZED.store(true, Ordering::Release);
    }
}

/// Allocates sub-page level memory in kernel space.
/// The size of the memory to allocate is specified in bits and
/// will be aligned to the next 8 byte boundary.
///
/// This function will return a pointer to the allocated memory
/// if successful, or None if the allocation failed.
///
/// The allocation will fail if:
/// - The size is 0
/// - The kernel memory system has not been initialized
pub fn kmalloc(size: usize) -> Option<*mut u8> {
    assert!(is_initialized(), "kernel memory system not initialized");
    assert!(size > 0, "allocation size must be greater than 0");

    unsafe {
        let aligned_size = align_up(size, 8) + size_of::<AllocList>();
        let mut head = KMEM_HEAD?;
        let tail = (KMEM_HEAD? as *mut u8).add(KMEM_ALLOCATED * get_page_size()) as *mut AllocList;

        while head < tail {
            if (*head).is_free() && aligned_size <= (*head).size() {
                let chunk_size = (*head).size();
                let remaining_size = chunk_size - aligned_size;
                (*head).set_taken();

                if remaining_size > size_of::<AllocList>() {
                    let new_head = (head as *mut u8).add(aligned_size) as *mut AllocList;
                    (*new_head).set_free();
                    (*new_head).set_size(remaining_size);
                    (*head).set_size(aligned_size);
                } else {
                    (*head).set_size(chunk_size);
                }

                return Some(head.add(1) as *mut u8);
            } else {
                // If we get here, what we saw wasn't free or big enough.
                // Move to the next node.
                head = (head as *mut u8).add((*head).size()) as *mut AllocList;
            }
        }
    }

    // If we get here, we couldn't find a free block.
    // TODO: Implement a way to allocate more memory.
    None
}

/// Allocates sub-page level memory in kernel space and zeroes it.
/// The size of the memory to allocate is specified in bits and
/// will be aligned to the next 8 byte boundary.
pub fn kzmalloc(size: usize) -> Option<*mut u8> {
    let align_size = align_up(size, 8);
    let ptr = kmalloc(align_size)?;

    for i in 0..align_size {
        unsafe {
            *ptr.add(i) = 0;
        }
    }

    Some(ptr)
}

/// Frees memory allocated by [`kmalloc`].
///
/// # Safety
/// This function will panic if the pointer is null.
pub fn kfree(ptr: *mut u8) {
    assert!(is_initialized(), "kernel memory system not initialized");
    assert!(!ptr.is_null(), "can not deallocate a null pointer");

    unsafe {
        let p = (ptr as *mut AllocList).offset(-1);

        if (*p).is_taken() {
            (*p).set_free();
        }

        // Coalesce the free blocks.
        // This tries to reduce fragmentation by merging adjacent free blocks.
        coalesce();
    }
}

/// Coalesces (Merges) adjacent free blocks in the kernel memory system.
/// This function is called after a block of memory is freed to reduce fragmentation.
///
/// # Safety
/// This function will panic if the kernel memory system has not been initialized.
fn coalesce() {
    unsafe {
        let mut head = KMEM_HEAD.expect("kernel memory system not initialized");
        let tail = (KMEM_HEAD.unwrap() as *mut u8).add(KMEM_ALLOCATED * get_page_size()) as *mut AllocList;

        while head < tail {
            let next = (head as *mut u8).add((*head).size()) as *mut AllocList;

            if (*head).size() == 0 {
                // If the size of the current node is 0, we have
                // a bad heap cause by a double free (I think).
                break;
            } else if next >= tail {
                // If the next node is out of bounds, we're done.
                break;
            } else if (*head).is_free() && (*next).is_free() {
                // If the current node and the next node are free,
                // we can merge them into one node.
                (*head).set_size((*head).size() + (*next).size());
            }

            head = (head as *mut u8).add((*head).size()) as *mut AllocList;
        }
    }
}

/// For debugging purposes, print the kmem table.
///
/// # Safety
/// This function will panic if the kernel memory system has not been initialized.
pub fn print_kmem_table() {
    unsafe {
        let mut head = KMEM_HEAD.expect("kernel memory system not initialized");
        let tail = (KMEM_HEAD.unwrap() as *mut u8).add(KMEM_ALLOCATED * get_page_size())
            as *mut AllocList;
        while head < tail {
            println!(
                "{:p}: Length = {:<10} Taken = {}",
                head,
                (*head).size(),
                (*head).is_taken()
            );
            head = (head as *mut u8).add((*head).size())
                as *mut AllocList;
        }
    }
}