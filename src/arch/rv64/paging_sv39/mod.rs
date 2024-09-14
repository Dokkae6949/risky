pub mod entry;
pub mod table;
pub mod mapping;

pub use table::*;
pub use entry::*;
use crate::arch::consts::{get_bss_end, get_bss_start, get_data_end, get_data_start, get_heap_size, get_heap_start, get_page_align, get_rodata_end, get_rodata_start, get_stack_end, get_stack_start, get_text_end, get_text_start};
use crate::arch::paging_sv39::mapping::id_map_range;
use crate::arch::rv64::asm::{write_satp};
use crate::arch::rv64::memory::kernel_allocator;

/// Initialize the virtual memory.
/// This function will identity map important memory regions.
/// # Safety
/// This function will panic if the kernel heap is not initialized.
pub fn init() {
    identity_map();

    let table = kernel_allocator::get_page_table().expect("failed to get root page table");

    write_satp((table as usize >> 12) | (8 << 60));
}

/// Identity map important memory regions.
/// # Safety
/// This function will panic if the kernel heap is not initialized.
#[inline(always)]
fn identity_map() {
    let root_ptr = kernel_allocator::get_page_table().expect("failed to get root page table");
    let mut root = unsafe { root_ptr.as_mut().expect("root is null") };
    let kheap_head = kernel_allocator::get_head().expect("failed to get kernel heap head") as usize;
    let kheap_pages = kernel_allocator::get_allocated_pages();

    // Map kernel heap
    id_map_range(
        &mut root,
        kheap_head,
        kheap_head + kheap_pages * 4096,
        EntryBits::ReadWrite.bits());

    // Map heap descriptors
    let heap_pages = get_heap_size() / get_page_align();
    id_map_range(
        &mut root,
        get_heap_start(),
        get_heap_start() + heap_pages * 4096,
        EntryBits::ReadWrite.bits());

    // Map executable section
    id_map_range(
        &mut root,
        get_text_start(),
        get_text_end(),
        EntryBits::ReadExecute.bits());

    // Map rodata section
    // We put the ROdata section into the text section, so they can
    // potentially overlap however, we only care that it's read
    // only.
    id_map_range(
        &mut root,
        get_rodata_start(),
        get_rodata_end(),
        EntryBits::ReadExecute.bits());

    // Map data section
    id_map_range(
        &mut root,
        get_data_start(),
        get_data_end(),
        EntryBits::ReadWrite.bits());

    // Map bss section
    id_map_range(
        &mut root,
        get_bss_start(),
        get_bss_end(),
        EntryBits::ReadWrite.bits());

    // Map kernel stack
    id_map_range(
        &mut root,
        get_stack_start(),
        get_stack_end(),
        EntryBits::ReadWrite.bits());

    kernel_allocator::set_page_table(root);
}