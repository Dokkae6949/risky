use core::{arch::asm, isize};

#[derive(Debug, Clone)]
pub enum SbiError {
    Success,
    Failed,
    NotSupported,
    InvalidParam,
    Denied,
    InvalidAddress,
    AlreadyAvailable,
    AlreadyStarted,
    AlreadyStopped,
    NoShmem,
    InvalidState,
    BadRange,
    Unknown(isize),
}

impl SbiError {
    pub fn new(error_code: isize) -> Self {
        match error_code {
            0 => Self::Success,
            -1 => Self::Failed,
            -2 => Self::NotSupported,
            -3 => Self::InvalidParam,
            -4 => Self::Denied,
            -5 => Self::InvalidAddress,
            -6 => Self::AlreadyAvailable,
            -7 => Self::AlreadyStarted,
            -8 => Self::AlreadyStopped,
            -9 => Self::NoShmem,
            -10 => Self::InvalidState,
            -11 => Self::BadRange,
            code => Self::Unknown(code),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SbiRet<T> {
    pub error: SbiError,
    pub value: T,
}

impl<T> SbiRet<T> {
    pub fn new(error: SbiError, value: T) -> Self {
        Self { error, value }
    }
}

#[derive(Debug, Clone)]
pub struct SbiCall {
    pub extension_id: usize,
    pub function_id: usize,
    pub args: [usize; 6],
}

impl SbiCall {
    pub fn new(extension_id: usize, function_id: usize, args: [usize; 6]) -> Self {
        Self {
            extension_id,
            function_id,
            args,
        }
    }

    pub unsafe fn call(&self) -> SbiRet<usize> {
        let (error, value);
        asm!(
            "ecall",
            in("a7") self.extension_id,
            in("a6") self.function_id,
            inlateout("a0") self.args[0] => error,
            inlateout("a1") self.args[1] => value,
            in("a2") self.args[2],
            in("a3") self.args[3],
            in("a4") self.args[4],
            in("a5") self.args[5],
        );
        SbiRet::new(SbiError::new(error), value)
    }
}
