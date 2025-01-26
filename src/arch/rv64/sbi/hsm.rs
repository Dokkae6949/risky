use super::{ecall::ecall3, sbi_ret::SbiRet};

#[inline(always)]
pub fn hart_start(hart_id: usize, start_address: usize, opaque: usize) -> SbiRet<usize> {
    unsafe { ecall3(0x48534D, 0, hart_id, start_address, opaque) }
}
