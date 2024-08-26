use spin::Mutex;
use crate::arch::logger::OpenSbiLogger;

pub static PRINT_LOCK: Mutex<OpenSbiLogger> = Mutex::new(OpenSbiLogger);

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        let mut logger = $crate::arch::macros::print::PRINT_LOCK.lock();
        let _ = write!(logger, $($args)*).unwrap();
    });
}

#[macro_export]
macro_rules! println {
    () => ({
        print!("\r\n")
    });
    ($fmt:expr) => ({
        print!(concat!($fmt, "\r\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        print!(concat!($fmt, "\r\n"), $($args)+)
    });
}