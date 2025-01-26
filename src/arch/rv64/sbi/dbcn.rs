use super::{ecall::ecall3, sbi_ret::SbiRet};

#[inline(always)]
pub fn debug_console_write(content: &str) -> SbiRet<usize> {
    unsafe { ecall3(0x4442434E, 0, content.len(), content.as_ptr() as usize, 0) }
}
