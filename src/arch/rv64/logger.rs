use core::fmt::Error;
use opensbi::debug_console_write_byte;
use crate::logger::Logger;

pub struct OpenSbiLogger;

impl Logger for OpenSbiLogger {
    fn write(&self, bytes: &[u8]) -> core::fmt::Result {
        for byte in bytes {
            match debug_console_write_byte(*byte).error {
                opensbi::call::SbiError::Success => {},
                _ => return Err(Error),
            }
        }

        Ok(())
    }
}

impl core::fmt::Write for OpenSbiLogger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes())
    }
}