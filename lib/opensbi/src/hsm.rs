use core::usize;

use crate::{ecall0, ecall1, ecall3, SbiRet};

pub const HSM_EID: usize = 0x48534D;
pub const HSM_EID_HART_START_FID: usize = 0;
pub const HSM_EID_HART_STOP_FID: usize = 1;
pub const HSM_EID_HART_GET_STATUS_FID: usize = 2;
pub const HSM_EID_HART_SUSPEND_FID: usize = 3;

pub mod suspend_type {
    pub const RETENTIVE: u32 = 0x00000000;
    pub const NON_RETENTIVE: u32 = 0x80000000;
}

#[derive(Debug, Clone, PartialEq)]
pub enum HartState {
    Started,
    Stopped,
    StartPending,
    StopPending,
    Suspended,
    SuspendPending,
    ResumePending,
    Unknown(usize),
}

impl HartState {
    pub fn new(state_id: usize) -> Self {
        match state_id {
            0 => Self::Started,
            1 => Self::Stopped,
            2 => Self::StartPending,
            3 => Self::StopPending,
            4 => Self::Suspended,
            5 => Self::SuspendPending,
            6 => Self::ResumePending,
            state => Self::Unknown(state),
        }
    }
}

#[inline(always)]
pub fn hart_start(
    hart_id: usize,
    start_address: usize,
    opaque: usize
) -> SbiRet<usize> {
    unsafe {
        ecall3(HSM_EID, HSM_EID_HART_START_FID, hart_id, start_address, opaque)
    }
}

#[inline(always)]
pub fn hart_stop() -> SbiRet<usize> {
    unsafe {
        ecall0(HSM_EID, HSM_EID_HART_STOP_FID)
    }
}

#[inline(always)]
pub fn hart_get_status(
    hart_id: usize,
) -> SbiRet<HartState> {
    let result = unsafe {
        ecall1(HSM_EID, HSM_EID_HART_GET_STATUS_FID, hart_id)
    };

    SbiRet::new(result.error, HartState::new(result.value))
}

#[inline(always)]
pub fn hart_suspend(
    suspend_type: u32,
    resume_address: usize,
    opaque: usize
) -> SbiRet<usize> {
    unsafe {
        ecall3(HSM_EID, HSM_EID_HART_SUSPEND_FID, suspend_type as _, resume_address, opaque)
    }
}
