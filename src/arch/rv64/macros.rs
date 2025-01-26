use spin::Mutex;

use super::OpenSbiLogger;

pub static PRINT_LOCK: Mutex<OpenSbiLogger> = Mutex::new(OpenSbiLogger);

#[allow(unused)]
macro_rules! print {
    ($($args:tt)+) => {{
        use core::fmt::Write;
        let mut logger = $crate::arch::rv64::macros::PRINT_LOCK.lock();
        let _ = logger.write_fmt(format_args!($($args)*));
    }};
}

#[allow(unused)]
macro_rules! println {
    () => ({
        print!("\r\n")
    });
    ($fmt:expr) => ({
        print!(concat!($fmt, "\r\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        print!(concat!($fmt, "\r\n"), $($args)*)
    });
}
