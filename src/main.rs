#![no_std]
#![no_main]

#[macro_use]
mod arch;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain_ap() -> ! {
    loop {}
}
