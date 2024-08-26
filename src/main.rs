#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
mod arch;
mod panic;
mod logger;
mod allocator;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    println!("Kernel started");

    loop {}
}

#[no_mangle]
pub extern "C" fn kmain_ap() -> ! {
    println!("Kernel started (AP)");

    loop {}
}
