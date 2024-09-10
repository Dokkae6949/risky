use core::arch::asm;
use core::arch::riscv64::wfi;
use opensbi::time::set_timer;
use crate::arch::rv64::asm::get_time;

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
            in(reg) s_mode_interrupt_handler as usize,
        );
    }
}

#[inline(always)]
fn get_interrupt_cause() -> usize {
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

/// Extract the interrupt bit and the cause from the scause register.
/// Returns `(is_interrupt: bool, cause: usize)`
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

#[no_mangle]
pub extern "riscv-interrupt-s" fn s_mode_interrupt_handler() {
    let (is_interrupt, cause) = extract_scause(get_interrupt_cause());

    if is_interrupt {
        println!("+ S-Mode Interrupt");
    } else {
        println!("+ S-Mode Exception");
    }

    println!("| Cause: {:#x}", cause);

    match (is_interrupt, cause) {
        (false, 0x2) => {
            println!("| Illegal instruction");
            panic!("Illegal instruction");
        }
        (false, 0x5) => {
            println!("| Load access fault");
            panic!("Load access fault");
        }
        (true, 0x5) => {
            println!("| Supervisor timer interrupt");
            clear_timer_interrupt();
            set_timer(get_time() + 10000000);
        }
        (false, 0x7) => {
            println!("| Store/AMO access fault");
            panic!("Store/AMO access fault");
        }
        _ => {
            println!("| Unhandled interrupt/exception");
            panic!("Unhandled interrupt/exception");
        }
    };
}