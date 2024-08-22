use core::usize;

use super::call::{SbiCall, SbiRet};

pub const HSM_EID: usize = 0x48534D;
pub const HSM_EID_HART_START_FID: usize = 0;
pub const HSM_EID_HART_STOP_FID: usize = 1;
pub const HSM_EID_HART_GET_STATUS_FID: usize = 2;
pub const HSM_EID_HART_SUSPEND_FID: usize = 3;

pub mod suspend_type {
    pub const RETENTIVE: u32 = 0x00000000;
    pub const NON_RETENTIVE: u32 = 0x80000000;
}

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

pub fn hart_start(
    hartid: usize,
    start_address: usize,
    opaque: usize
) -> SbiRet<usize> {
    unsafe {
        SbiCall::new(
            HSM_EID,
            HSM_EID_HART_START_FID,
            [hartid, start_address, opaque, 0, 0, 0]
        ).call()
    }
}

pub fn hart_stop() -> SbiRet<usize> {
    unsafe {
        SbiCall::new(
            HSM_EID,
            HSM_EID_HART_STOP_FID,
            [0, 0, 0, 0, 0, 0]
        ).call()
    }
}

pub fn hart_get_status(
    hartid: usize,
) -> SbiRet<HartState> {
    let result = unsafe {
        SbiCall::new(
            HSM_EID,
            HSM_EID_HART_GET_STATUS_FID,
            [hartid, 0, 0, 0, 0, 0]
        ).call()
    };

    SbiRet::new(result.error, HartState::new(result.value))
}

pub fn hart_suspend(
    suspend_type: u32,
    resume_address: usize,
    opaque: usize
) -> SbiRet<usize> {
    unsafe {
        SbiCall::new(
            HSM_EID,
            HSM_EID_HART_SUSPEND_FID,
            [suspend_type as _, resume_address, opaque, 0, 0, 0]
        ).call()
    }
}
