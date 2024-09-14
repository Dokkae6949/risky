mod handler;
mod frame;

use core::arch::asm;
use core::arch::riscv64::wfi;
use crate::arch::trap::handler::s_mode_trap_handler;

#[inline(always)]
pub fn halt() {
    unsafe {
        wfi();
    }
}

#[inline(always)]
pub fn enable_timer_interrupts() {
    unsafe {
        asm!(
            "li t0, 32",
            "csrs sie, t0",
            options(nomem, nostack),
        );
    }
}

#[inline(always)]
pub fn clear_timer_interrupt() {
    unsafe {
        asm!(
            "li t0, 32",
            "csrc sip, t0",
            options(nomem, nostack),
        );
    }
}

#[inline(always)]
pub fn disable_timer_interrupts() {
    unsafe {
        asm!(
            "li t0, 32",
            "csrc sie, t0",
            options(nomem, nostack),
        );
    }
}

#[inline(always)]
pub fn enable_s_mode_interrupts() {
    unsafe {
        asm!(
            "csrw stvec, {}",
            "csrsi sstatus, 2",
            options(nomem, nostack),
            in(reg) s_mode_trap_handler as usize,
        );
    }
}

#[inline(always)]
fn get_trap_cause() -> usize {
    let mut cause: usize;
    unsafe {
        asm!(
            "csrr {}, scause",
            lateout(reg) cause,
            options(nomem, nostack),
        );
    }
    cause
}

/// Extract the trap bit and the cause from the scause register.
/// Returns `(is_async: bool, cause: usize)`
#[inline(always)]
pub fn extract_scause(scause: usize) -> (bool, usize) {
    let interrupt = (scause >> (core::mem::size_of::<usize>() * 8 - 1)) != 0;
    let code = scause & !(1 << (core::mem::size_of::<usize>() * 8 - 1));
    (interrupt, code)
}

#[inline(always)]
fn read_sepc() -> usize {
    let sepc: usize;
    unsafe {
        asm!("csrr {}, sepc", out(reg) sepc);
    }
    sepc
}

#[inline(always)]
fn write_sepc(sepc: usize) {
    unsafe {
        asm!("csrw sepc, {}", in(reg) sepc);
    }
}

