use core::arch::global_asm;

#[macro_use]
pub mod macros;

pub mod logger;
pub use logger::*;
pub mod sbi;

global_asm!(include_str!("asm/boot.s"));
