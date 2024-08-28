#![allow(dead_code)]

pub const KERNEL_ENTRY: usize = 0x8020_0000;

pub fn print_consts() {
    println!("+ Arch constants");
    println!("| Kernel Entry: {:#x}", KERNEL_ENTRY);
}