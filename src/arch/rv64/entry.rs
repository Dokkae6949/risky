use alloc::boxed::Box;
use core::fmt::Write;
use core::arch::{asm, global_asm};
use core::ops::Deref;
use opensbi::{hart_get_status, hart_start, HartState};
use crate::allocator;
use crate::arch::logger::OpenSbiLogger;
use crate::arch::macros::print::PRINT_LOCK;
use crate::arch::rv64::asm::{get_hart_id, init_stack_pointer};
use crate::logger::LOGGER;

global_asm!(include_str!("asm/boot.S"));

/// Test of zero values in BSS.
static BSS_TEST_ZERO: usize = 0;
/// Test of non-zero values in data.
static DATA_TEST_NONZERO: usize = 0xFFFF_FFFF_FFFF_FFFF;

#[no_mangle]
pub unsafe extern "C" fn kentry(hart_id: usize) -> ! {
    assert_eq!(BSS_TEST_ZERO, 0);
    assert_eq!(DATA_TEST_NONZERO, 0xFFFF_FFFF_FFFF_FFFF);

    println!("+ Booting RiskyOS ");
    println!("| Started on Hart: {}", hart_id);

    println!("Initializing allocator...");
    allocator::init();
    println!("Allocator initialized");

    println!("Initializing logger...");
    LOGGER.set_logger(Box::new(OpenSbiLogger));
    println!("Logger initialized");

    println!("funny");
    println!("+ Starting other harts...");
    for hid in 0..4 {
        let result = hart_start(hid, kentry_ap as *const fn() as usize, 0xc0ffee);
        println!("| Starting Hart {}: {:?}", hid, result);
    }

    crate::kmain();
}

#[no_mangle]
pub unsafe extern "C" fn kentry_ap() -> ! {
    init_stack_pointer();

    let hart_id = get_hart_id();

    println!("Hart {} started (AP)", hart_id);

    crate::kmain_ap();
}