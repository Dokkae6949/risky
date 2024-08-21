#![no_std]
#![no_main]

use core::arch::global_asm;

#[macro_use]
mod print;
mod panic;
mod uart;

global_asm!(include_str!("asm/trap.s"));
global_asm!(include_str!("asm/boot.s"));

#[no_mangle]
extern "C" fn main() -> ! {
    let mut serial_uart = uart::Uart::new(0x1000_0000);
    serial_uart.init();

    println!("Hello World!");

    loop {
        if let Some(c) = serial_uart.get() {
            match c {
                8 | 127 => print!("{}{}{}", 8 as char, ' ', 8 as char),
                10 | 13 => println!(),
                _ => print!("{}", c as char)
            }
        }
    }
}

#[no_mangle]
extern "C" fn main_hart() -> ! {
    println!("Hello Hart!");

    loop {}
}
