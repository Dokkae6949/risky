#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

use spin::Mutex;

#[macro_use]
mod arch;

pub static PRINT_LOCK: Mutex<OpenSbiLogger> = Mutex::new(OpenSbiLogger);

#[allow(unused)]
macro_rules! print {
    ($($args:tt)+) => {{
        use core::fmt::Write;
        let mut logger = $crate::PRINT_LOCK.lock();
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

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main_mp(hart_id: usize) -> bool {
    println!("Hello World from {}", hart_id);

    if hart_id == 0 {
        return true;
    }

    false
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct SbiRet {
    pub error: usize,
    pub value: usize,
}

impl SbiRet {
    pub fn new(error: usize, value: usize) -> Self {
        Self { error, value }
    }
}

#[inline(always)]
pub unsafe fn sbi_ecall(extension: usize, function: usize, param: [usize; 6]) -> SbiRet {
    let (error, value);

    asm!(
        "ecall",
        in("a7") extension,
        in("a6") function,
        inlateout("a0") param[0] => error,
        inlateout("a1") param[1] => value,
        in("a2") param[2],
        in("a3") param[3],
        in("a4") param[4],
        in("a5") param[5],
    );

    SbiRet::new(error, value)
}

#[inline(always)]
pub fn debug_console_write(content: &str) -> SbiRet {
    unsafe {
        sbi_ecall(
            0x4442434E,
            0,
            [content.len(), content.as_ptr() as usize, 0, 0, 0, 0],
        )
    }
}

pub struct OpenSbiLogger;

impl OpenSbiLogger {
    fn write(&self, content: &str) -> core::fmt::Result {
        match debug_console_write(content).error {
            0 => Ok(()),
            _ => Err(core::fmt::Error),
        }
    }
}

impl core::fmt::Write for OpenSbiLogger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s)
    }
}
