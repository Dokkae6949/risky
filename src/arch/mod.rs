#[cfg(target_arch = "riscv64")]
#[macro_use]
pub mod rv64;
#[cfg(target_arch = "riscv64")]
pub use rv64::*;