#![allow(dead_code)]

extern "C" {
    static MEMORY_START: usize;
    static MEMORY_END: usize;
    static STACK_START: usize;
    static STACK_END: usize;
    static HEAP_START: usize;
    static HEAP_SIZE: usize;

    pub fn _kentry();
}

/// Returns the address of the kernel memory start.
#[inline(always)]
pub fn get_memory_start() -> usize {
    unsafe { MEMORY_START }
}

/// Returns the address of the kernel memory end.
#[inline(always)]
pub fn get_memory_end() -> usize {
    unsafe { MEMORY_END }
}

/// Returns the address of the kernel stack start.
#[inline(always)]
pub fn get_stack_start() -> usize {
    unsafe { STACK_START }
}

/// Returns the address of the kernel stack end.
#[inline(always)]
pub fn get_stack_end() -> usize {
    unsafe { STACK_END }
}

/// Returns the size of the kernel stack in bytes.
#[inline(always)]
pub fn get_stack_size() -> usize {
    unsafe { STACK_END - STACK_START }
}

/// Returns the address of the kernel heap start.
#[inline(always)]
pub fn get_heap_start() -> usize {
    unsafe { HEAP_START }
}

/// Returns the size of the kernel heap in bytes.
#[inline(always)]
pub fn get_heap_size() -> usize {
    unsafe { HEAP_SIZE }
}

/// Returns the size of all pages in bytes.
#[inline(always)]
pub fn get_pages_size() -> usize {
    get_page_size() * 8 * 3
}

/// Returns the alignment of the pages.
#[inline(always)]
pub fn get_page_align() -> usize {
    4096
}

/// Returns the size of a page in bytes.
#[inline(always)]
pub fn get_page_size() -> usize {
    512
}

pub fn print_consts() {
    println!("+ Arch constants");
    println!("| Kernel Entry: {:#x}", _kentry as usize);
    println!("| Memory Start: {:#x}", get_memory_start());
    println!("| Memory End: {:#x}", get_memory_end());
    println!("| Stack Start: {:#x}", get_stack_start());
    println!("| Stack End: {:#x}", get_stack_end());
    println!("| Stack Size: {:#x}", get_stack_size());
    println!("| Heap Start: {:#x}", get_heap_start());
    println!("| Heap Size: {:#x}", get_heap_size());
    println!("| Pages Size: {:#x}", get_pages_size());
    println!("| Page Align: {:#x}", get_page_align());
    println!("| Page Size: {:#x}", get_page_size());
}