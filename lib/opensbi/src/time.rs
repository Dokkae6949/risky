use crate::{ecall1, SbiRet};

pub const TIME_EID: usize = 0x54494D45;
pub const TIME_EID_SET_TIMER_FID: usize = 0;

#[inline(always)]
pub fn set_timer(stime_value: u64) -> SbiRet<usize> {
    unsafe {
        ecall1(TIME_EID, TIME_EID_SET_TIMER_FID, stime_value as usize)
    }
}