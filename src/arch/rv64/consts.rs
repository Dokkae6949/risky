#![allow(dead_code)]

extern "C" {
    static _stack_end: usize;
    static _memory_end: usize;
    static _heap_start: usize;
    static _heap_size: usize;

    pub fn _kentry();
}

pub fn get_stack_end() -> usize {
    unsafe { &_stack_end as *const _ as usize }
}

pub fn get_memory_end() -> usize {
    unsafe { &_memory_end as *const _ as usize }
}

pub fn get_heap_start() -> usize {
    unsafe { &_heap_start as *const _ as usize }
}

pub fn get_heap_size() -> usize {
    unsafe { &_heap_size as *const _ as usize }
}

pub fn print_consts() {
    println!("+ Arch constants");
    println!("| Kernel Entry: {:#x}", _kentry as usize);
    println!("| Stack End: {:#x}", get_stack_end());
    println!("| Memory End: {:#x}", get_memory_end());
    println!("| Heap Start: {:#x}", get_heap_start());
    println!("| Heap Size: {:#x}", get_heap_size());
}