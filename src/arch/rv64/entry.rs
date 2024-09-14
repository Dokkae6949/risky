use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::arch::{asm, global_asm};
use opensbi::hart_start;
use opensbi::time::set_timer;
use crate::allocator;
use crate::arch::consts::{print_consts, _kentry, get_heap_size, get_page_align, get_heap_start, get_text_start, get_text_end, get_rodata_start, get_rodata_end, get_data_start, get_data_end, get_bss_start, get_bss_end, get_stack_start, get_stack_end};
use crate::arch::logger::OpenSbiLogger;
use crate::arch::paging_sv39::{id_map_range, EntryBits};
use crate::arch::rv64::asm::{enable_virtual_memory_sv39, get_hart_id, get_time, init_stack_pointer, is_virtual_memory_enabled, read_satp};
use crate::arch::rv64::trap::{enable_s_mode_interrupts, enable_timer_interrupts};
use crate::arch::rv64::memory;
use crate::arch::rv64::memory::{kernel_allocator, page, page_allocator};
use crate::arch::rv64::memory::kernel_allocator::{kzmalloc, print_kmem_table};
use crate::arch::rv64::memory::page_allocator::print_page_allocations;
use crate::logger::LOGGER;

global_asm!(include_str!("asm/memory.S"));
global_asm!(include_str!("asm/boot.S"));

/// Test of zero values in BSS.
static BSS_TEST_ZERO: usize = 0;
/// Test of non-zero values in data.
static DATA_TEST_NONZERO: usize = 0xFFFF_FFFF_FFFF_FFFF;

#[no_mangle]
pub unsafe extern "C" fn kentry(hart_id: usize, dtb: usize) -> ! {
    assert_eq!(BSS_TEST_ZERO, 0);
    assert_eq!(DATA_TEST_NONZERO, 0xFFFF_FFFF_FFFF_FFFF);

    enable_s_mode_interrupts();

    println!("+ Booting RiskyOS ");
    println!("| Made by: DokkaeCat <linfia21@htl-kaindorf.at>");
    println!("| Started on Hart: {}", hart_id);
    println!("| Device Tree Blob at: {:#x}", dtb);

    print_consts();

    println!("Initializing page allocator...");
    page_allocator::init();
    println!("Page allocator initialized");

    println!("Initializing kernel memory...");
    kernel_allocator::init();
    println!("Kernel memory initialized");

    println!("+ Initializing virtual memory...");
    identity_map();
    enable_virtual_memory_sv39(kernel_allocator::get_page_table().expect("failed to get root page table"));
    println!("| Virtual Memory Enabled: {}", is_virtual_memory_enabled());
    println!("| satp: {:#x}", read_satp());

    // Requires HEAP to be initialized.
    println!("Initializing logger...");
    LOGGER.set_logger(Box::new(OpenSbiLogger));
    println!("Logger initialized");

    println!("+ Starting other harts...");
    for hid in 0..4 {
        // 0xc0ffee is the argument passed to the kernel entry point inside the a1 register.
        // This is a weird workaround because the hart should already
        // be inside kentry_ap but instead it starts executing way before that.
        // So we also check for the a1 register to be 0xc0ffee and if it is we call kentry_ap manually.
        let result = hart_start(hid, _kentry as usize, 0xc0ffee);
        println!("| Starting Hart {}: {:?}", hid, result);
    }

    crate::kmain();
}

#[no_mangle]
pub unsafe extern "C" fn kentry_ap() -> ! {
    let hart_id = get_hart_id();

    println!("Hart {} started (AP)", hart_id);

    crate::kmain_ap();
}

/// Identity map important memory regions.
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