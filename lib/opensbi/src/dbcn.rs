// Documentation can be found here:
// https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/src/ext-debug-console.adoc

use core::usize;
use crate::{ecall1, ecall3, SbiRet, split_address};

pub const DBCN_EID: usize = 0x4442434E;
pub const DBCN_EID_WRITE_FID: usize = 0;
pub const DBCN_EID_READ_FID: usize = 1;
pub const DBCN_EID_WRITE_BYTE_FID: usize = 2;


#[inline(always)]
pub fn debug_console_write(data: &[u8])-> SbiRet<usize> {
    let (base_addr_lo, base_addr_hi) = split_address(data.as_ptr() as usize);

    unsafe {
        ecall3(DBCN_EID, DBCN_EID_WRITE_FID, data.len(), base_addr_lo, base_addr_hi)
    }
}

#[inline(always)]
pub fn debug_console_read(data: &mut [u8])-> SbiRet<usize> {
    let (base_addr_lo, base_addr_hi) = split_address(data.as_mut_ptr() as usize);

    unsafe {
        ecall3(DBCN_EID, DBCN_EID_READ_FID, data.len(), base_addr_lo, base_addr_hi)
    }
}

#[inline(always)]
pub fn debug_console_write_byte(byte: u8) -> SbiRet<usize> {
    unsafe {
        ecall1(DBCN_EID, DBCN_EID_WRITE_BYTE_FID, byte as _)
    }
}