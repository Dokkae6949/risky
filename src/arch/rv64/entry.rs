use alloc::boxed::Box;
use alloc::vec::Vec;
use core::arch::{asm, global_asm};
use opensbi::hart_start;
use opensbi::time::set_timer;
use crate::allocator;
use crate::arch::consts::{print_consts, _kentry};
use crate::arch::logger::OpenSbiLogger;
use crate::arch::rv64::asm::{get_hart_id, get_time, init_stack_pointer, is_virtual_memory_enabled, read_satp};
use crate::arch::rv64::interrupt::{enable_s_mode_interrupts, enable_timer_interrupts};
use crate::arch::rv64::memory;
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
    println!("| Virtual Memory Enabled: {}", is_virtual_memory_enabled());
    println!("| satp: {:#x}", read_satp());

    print_consts();

    println!("Initializing page allocator...");
    memory::page_allocator::init();
    println!("Page allocator initialized");

    println!("Initializing kernel memory...");
    memory::kernel_allocator::init();
    println!("Kernel memory initialized");

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

