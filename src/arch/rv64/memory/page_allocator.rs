use core::ptr::write_bytes;
use core::sync::atomic::{AtomicBool, Ordering};
pub use crate::arch::rv64::memory::page::*;
use crate::allocator::align_up;
use crate::arch::consts::{get_heap_size, get_heap_start, get_page_align, get_pages_size, get_pages_start};
use crate::arch::rv64::memory::page::{Page, PageBits};

/// [`get_page_align`] aligned pointer to the start of the heap.
static mut ALLOC_START: usize = 0;
static mut IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Check if the page allocator is initialized.
pub fn is_initialized() -> bool {
    unsafe {
        IS_INITIALIZED.load(Ordering::SeqCst)
    }
}

/// Initializes the allocation system.
/// This function should be called only once.
/// It clears all pages and sets the start of the heap.
pub fn init() {
    unsafe {
        assert!(!is_initialized(), "the page allocator has already been initialized");

        let max_pages = get_heap_size() / get_page_align();
        let ptr = get_heap_start() as *mut Page;

        println!("Clearing {} pages", max_pages);
        for i in 0..max_pages {
            println!("Clearing page {} | {:?}", i, (*ptr.add(i)));
            (*ptr.add(i)).clear();
        }
        println!("All pages cleared");

        // This is needed because the first page is used for the page descriptor.
        // After that come the actual pages.
        ALLOC_START = align_up(get_heap_start(), get_page_align());
        IS_INITIALIZED.store(true, Ordering::SeqCst);
    }
}

/// Allocates a number of pages.
/// Returns a pointer to the first page.
/// If no suitable pages are found, it returns None.
/// # Safety
/// This function must only be called after the memory has been initialized.
/// The number of pages must be greater than 0.
/// If either of these conditions is not met, the function will panic.
pub fn alloc(pages: usize) -> Option<*mut u8> {
    assert!(is_initialized(), "the page allocator was not initialized");
    assert!(pages > 0);

    unsafe {
        let max_pages = get_heap_size() / get_page_align();
        let ptr = get_heap_start() as *mut Page;

        println!("Allocating {}/{} pages", pages, max_pages);

        for i in 0..max_pages - pages {
            let mut found = false;

            // Check if the current page is free.
            // If it is, we found the first candidate.
            if (*ptr.add(i)).is_free() {
                found = true;
                // Check if enough consecutive pages are free.
                for j in i..i + pages {
                    // If the page is taken, we break the loop
                    // and start searching for a new candidate.
                    if (*ptr.add(j)).is_taken() {
                        found = false;
                        break;
                    }
                }
            }

            if found {
                for j in i..i + pages - 1 {
                    (*ptr.add(j)).set_flags(PageBits::Taken);
                }

                // Mark the last page as taken and last.
                (*ptr.add(i + pages - 1)).set_flags(PageBits::TakenLast);

                // The page structures aren't useful on their own.
                // Instead, we convert the index to a pointer to the first byte of the page.
                return Some((ALLOC_START + get_page_align() * i) as *mut u8);
            }
        }
    }

    None
}

/// Frees a number of pages.
/// # Safety
/// This function must only be called after the memory has been initialized.
/// The number of pages must be greater than 0.
/// If either of these conditions is not met, the function will panic.
pub fn zalloc(pages: usize) -> Option<*mut u8> {
    let ptr = alloc(pages)?;
    let size = (get_page_align() * pages) / 64;
    let big_ptr = ptr as *mut u64;

    for i in 0..size {
        unsafe {
            (*big_ptr.add(i)) = 0;
        }
    }

    Some(ptr)
}

/// Frees a number of pages.
/// # Safety
/// This function must only be called after the memory has been initialized.
/// The pointer must be a valid pointer to the first byte of a page.
/// If either of these conditions is not met, the function will panic.
pub fn dealloc(ptr: *mut u8) {
    assert!(is_initialized(), "the page allocator was not initialized");
    assert!(!ptr.is_null(), "can not deallocate a null pointer");

    unsafe {
        let addr = get_heap_start() + (ptr as usize - ALLOC_START) / get_page_align();

        assert!(addr >= get_heap_start() && addr < get_heap_start() + get_heap_size(), "pointer is out of bounds");

        let mut p = addr as *mut Page;

        while (*p).is_taken() && !(*p).is_last() {
            (*p).clear();
            p = p.add(1);
        }

        assert_eq!((*p).is_last(), true, "possible double-free detected");

        // If we get here, all previous pages were taken
        // and the last page is the only one left.
        (*p).clear();
    }
}


/// Print all page allocations
/// This is mainly used for debugging.
pub fn print_page_allocations() {
    unsafe {
        let num_pages = get_heap_size() / get_page_align();
        let mut beg = get_heap_start() as *const Page;
        let end = beg.add(num_pages);
        let alloc_beg = ALLOC_START;
        let alloc_end = ALLOC_START + num_pages * get_page_align();
        println!();
        println!(
            "PAGE ALLOCATION TABLE\nMETA: {:p} -> {:p}\nPHYS: \
					0x{:x} -> 0x{:x}",
            beg, end, alloc_beg, alloc_end
        );
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        let mut num = 0;
        while beg < end {
            if (*beg).is_taken() {
                let start = beg as usize;
                let memaddr = ALLOC_START
                    + (start - get_heap_start())
                    * get_page_align();
                print!("0x{:x} => ", memaddr);
                loop {
                    num += 1;
                    if (*beg).is_last() {
                        let end = beg as usize;
                        let memaddr = ALLOC_START
                            + (end
                            - get_heap_start())
                            * get_page_align()
                            + get_page_align() - 1;
                        print!(
                            "0x{:x}: {:>3} page(s)",
                            memaddr,
                            (end - start + 1)
                        );
                        println!(".");
                        break;
                    }
                    beg = beg.add(1);
                }
            }
            beg = beg.add(1);
        }
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        println!(
            "Allocated: {:>6} pages ({:>10} bytes).",
            num,
            num * get_page_align()
        );
        println!(
            "Free     : {:>6} pages ({:>10} bytes).",
            num_pages - num,
            (num_pages - num) * get_page_align()
        );
        println!();
    }
}