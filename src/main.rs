#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[macro_use]
mod arch;

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main_mp() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
