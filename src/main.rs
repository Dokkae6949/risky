#![no_std]
#![no_main]

use core::arch::global_asm;

use sbi::{dbcn::debug_console_read, hsm::hart_start};

#[macro_use]
mod print;
mod panic;
mod sbi;

global_asm!(include_str!("asm/boot.S"));

#[no_mangle]
extern "C" fn main() -> ! {
    println!("Hello World!");

    let result = hart_start(1, main_hart as _, 0);
    println!("{:?}", result);

    loop {
        let mut input: [u8; 1] = [0];
        let result = debug_console_read(&mut input);
        match input[0] {
            0 => print!(""),
            13 => println!(),
            127 => print!("{}{}{}", 8 as char, ' ', 8 as char),
            value => print!("{}", value as char)
        };
    }
}

#[no_mangle]
extern "C" fn main_hart() -> ! {
    println!("Hello Hart!");

    loop {}
}
