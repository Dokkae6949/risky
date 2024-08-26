#![no_std]

pub mod call;
pub mod dbcn;
pub mod hsm;

pub use call::*;
pub use dbcn::*;
pub use hsm::*;

/// Returns the address split up into (lo, hi)
#[inline(always)]
pub fn split_address(address: usize) -> (usize, usize) {
    if cfg!(target_pointer_width = "64") {
        (address, 0)
    } else if cfg!(target_pointer_width = "32") {
        (address & 0xFFFF_FFFF, address >> 32)
    } else {
        unimplemented!()
    }
}
