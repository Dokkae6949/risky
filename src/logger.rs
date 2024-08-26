use alloc::boxed::Box;
use spin::Mutex;

pub static mut LOGGER: LoggerWrapper = LoggerWrapper::new();

pub trait Logger: Send {
    fn write(&self, bytes: &[u8]) -> core::fmt::Result;
}

pub struct LoggerWrapper {
    logger: Mutex<Option<Box<dyn Logger>>>,
}

impl LoggerWrapper {
    pub const fn new() -> Self {
        Self {
            logger: Mutex::new(None),
        }
    }

    pub fn set_logger(&self, logger: Box<dyn Logger + Send>) {
        *self.logger.lock() = Some(logger);
    }

    pub fn write(&self, bytes: &[u8]) -> core::fmt::Result {
        if let Some(ref logger) = *self.logger.lock() {
            logger.write(bytes)
        } else {
            Err(core::fmt::Error)
        }
    }
}

impl core::fmt::Write for LoggerWrapper {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes())
    }
}