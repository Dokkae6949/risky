#![allow(dead_code)]

extern "C" {
    static TEXT_START: usize;
    static TEXT_END: usize;
    static RODATA_START: usize;
    static RODATA_END: usize;
    static DATA_START: usize;
    static DATA_END: usize;
    static BSS_START: usize;
    static BSS_END: usize;
    static MEMORY_START: usize;
    static MEMORY_END: usize;
    static STACK_START: usize;
    static STACK_END: usize;
    static HEAP_START: usize;
    static HEAP_SIZE: usize;

    pub fn _kentry();
}

/// Returns the address of the kernel text start.
#[inline(always)]
pub fn get_text_start() -> usize {
    unsafe { TEXT_START }
}
/// Returns the address of the kernel text end.
#[inline(always)]
pub fn get_text_end() -> usize {
    unsafe { TEXT_END }
}
/// Returns the address of the kernel read-only data start.
#[inline(always)]
pub fn get_rodata_start() -> usize {
    unsafe { RODATA_START }
}
/// Returns the address of the kernel read-only data end.
#[inline(always)]
pub fn get_rodata_end() -> usize {
    unsafe { RODATA_END }
}
/// Returns the address of the kernel data start.
#[inline(always)]
pub fn get_data_start() -> usize {
    unsafe { DATA_START }
}
/// Returns the address of the kernel data end.
#[inline(always)]
pub fn get_data_end() -> usize {
    unsafe { DATA_END }
}
/// Returns the address of the kernel BSS start.
#[inline(always)]
pub fn get_bss_start() -> usize {
    unsafe { BSS_START }
}
/// Returns the address of the kernel BSS end.
#[inline(always)]
pub fn get_bss_end() -> usize {
    unsafe { BSS_END }
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