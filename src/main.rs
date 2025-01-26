#![no_std]
#![no_main]

use core::panic::PanicInfo;

use arch::sbi::hsm::hart_start;

#[macro_use]
mod arch;

extern "C" {
    pub fn _start();
}

#[no_mangle]
pub extern "C" fn main(hart_id: usize) -> ! {
    println!("Primary hart {} booted", hart_id);

    for hid in 1..4 {
        hart_start(hid, _start as usize, 0xc0ffee);
    }

    loop {}
}

#[no_mangle]
pub extern "C" fn main_mp(hart_id: usize) -> bool {
    println!("Booting hart {}", hart_id);

    if hart_id == 0 {
        return true;
    }

    hart_start(0, _start as usize, 0xc0ffee);

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Paniced: {}", info);

    loop {}
}
