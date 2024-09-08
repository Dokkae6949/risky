#![allow(dead_code)]

extern "C" {
    static STACK_START: usize;
    static STACK_END: usize;
    static HEAP_START: usize;
    static HEAP_SIZE: usize;
    static PAGES_START: usize;
    static PAGES_SIZE: usize;
    static PAGE_ALIGN: usize;
    static PAGE_SIZE: usize;

    pub fn _kentry();
}

/// Safe wrapper around the stack start constant.
#[inline(always)]
pub fn get_stack_start() -> usize {
    unsafe { &STACK_START as *const _ as usize }
}

/// Safe wrapper around the stack end constant.
#[inline(always)]
pub fn get_stack_end() -> usize {
    unsafe { &STACK_END as *const _ as usize }
}

/// Safe wrapper around the heap start constant.
#[inline(always)]
pub fn get_heap_start() -> usize {
    unsafe { &HEAP_START as *const _ as usize }
}

/// Safe wrapper around the stack end constant.
#[inline(always)]
pub fn get_heap_size() -> usize {
    unsafe { &HEAP_SIZE as *const _ as usize }
}

/// Safe wrapper around the stack end constant.
#[inline(always)]
pub fn get_pages_start() -> usize {
    unsafe { &PAGES_START as *const _ as usize }
}

/// Safe wrapper around the stack end constant.
#[inline(always)]
pub fn get_pages_size() -> usize {
    unsafe { PAGES_SIZE }
}

/// Safe wrapper around the stack end constant.
#[inline(always)]
pub fn get_page_align() -> usize {
    unsafe { PAGE_ALIGN }
}

/// Safe wrapper around the stack end constant.
#[inline(always)]
pub fn get_page_size() -> usize {
    unsafe { PAGE_SIZE }
}

pub fn print_consts() {
    println!("+ Arch constants");
    println!("| Kernel Entry: {:#x}", _kentry as usize);
    println!("| Stack Start: {:#x}", get_stack_start());
    println!("| Stack End: {:#x}", get_stack_end());
    println!("| Stack Size: {:#x}", get_stack_end() - get_stack_start());
    println!("| Heap Start: {:#x}", get_heap_start());
    println!("| Heap Size: {:#x}", get_heap_size());
    println!("| Pages Start: {:#x}", get_pages_start());
    println!("| Pages Size: {:#x}", get_pages_size());
    println!("| Page Align: {:#x}", get_page_align());
    println!("| Page Size: {:#x}", get_page_size());
}