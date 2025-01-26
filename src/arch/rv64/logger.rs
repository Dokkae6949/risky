use super::sbi::{dbcn::debug_console_write, sbi_ret::SbiRetKind};

pub struct OpenSbiLogger;

impl OpenSbiLogger {
    fn write(&self, content: &str) -> core::fmt::Result {
        match debug_console_write(content).kind {
            SbiRetKind::Success => Ok(()),
            _ => Err(core::fmt::Error),
        }
    }
}

impl core::fmt::Write for OpenSbiLogger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s)
    }
}
