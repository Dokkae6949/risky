#[derive(Debug, Clone, PartialEq)]
pub enum SbiRetKind {
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

impl SbiRetKind {
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

    pub fn code(&self) -> isize {
        match self {
            SbiRetKind::Success => 0,
            SbiRetKind::Failed => -1,
            SbiRetKind::NotSupported => -2,
            SbiRetKind::InvalidParam => -3,
            SbiRetKind::Denied => -4,
            SbiRetKind::InvalidAddress => -5,
            SbiRetKind::AlreadyAvailable => -6,
            SbiRetKind::AlreadyStarted => -7,
            SbiRetKind::AlreadyStopped => -8,
            SbiRetKind::NoShmem => -9,
            SbiRetKind::InvalidState => -10,
            SbiRetKind::BadRange => -11,
            SbiRetKind::Unknown(code) => *code,
        }
    }
}

impl From<isize> for SbiRetKind {
    fn from(value: isize) -> Self {
        Self::new(value)
    }
}

impl From<SbiRetKind> for isize {
    fn from(value: SbiRetKind) -> Self {
        value.code()
    }
}

#[derive(Debug, Clone)]
pub struct SbiRet<T> {
    pub kind: SbiRetKind,
    pub value: T,
}

impl<T> SbiRet<T> {
    pub fn new(kind: SbiRetKind, value: T) -> Self {
        Self { kind, value }
    }
}

impl<T> From<SbiRet<T>> for SbiRetKind {
    fn from(value: SbiRet<T>) -> Self {
        value.kind
    }
}
