use core::arch::asm;
use opensbi::time::set_timer;
use crate::arch::rv64::asm::get_time;
use crate::arch::trap::{clear_timer_interrupt, extract_scause, get_interrupt_cause, read_sepc};

#[no_mangle]
pub extern "riscv-interrupt-s" fn s_mode_trap_handler() {
    let (is_async, cause) = extract_scause(get_interrupt_cause());
    let epc = read_sepc();
    let stval = {
        let mut stval: usize;
        unsafe {
            asm!("csrr {}, stval", out(reg) stval);
        }
        stval
    };

    let return_pc = if is_async {
        s_mode_async_handler(cause, epc, stval)
    } else {
        s_mode_sync_handler(cause, epc, stval)
    };

    unsafe {
        asm!("csrw sepc, {}", in(reg) return_pc);
    }
}

#[inline(always)]
fn s_mode_async_handler(
    cause: usize,
    epc: usize,
    _tval: usize,
) -> usize {
    let mut return_pc = epc;

    match cause {
        1 => {
            // Supervisor software
            println!("Supervisor software interrupt");
        },
        3 => {
            // Machine software
            println!("Machine software interrupt");
        },
        5 => {
            // Supervisor timer
            println!("Supervisor timer interrupt");
            clear_timer_interrupt();
            set_timer(get_time() + 10_000_000);
        },
        7 => unsafe {
            // Machine timer
            println!("Machine timer interrupt");
            let mtimecmp = 0x0200_4000 as *mut u64;
            let mtime = 0x0200_bff8 as *const u64;
            // The frequency given by QEMU is 10_000_000 Hz, so this sets
            // the next interrupt to fire one second from now.
            mtimecmp.write_volatile(mtime.read_volatile() + 10_000_000);
        },
        9 => {
            // Supervisor external (interrupt from Platform Interrupt Controller (PLIC))
            println!("Supervisor external interrupt");
        },
        11 => {
            // Machine external (interrupt from Platform Interrupt Controller (PLIC))
            println!("Machine external interrupt");
        },
        _ => {
            panic!("Unhandled async trap -> {}", cause);
        }
    }

    return_pc
}

#[inline(always)]
fn s_mode_sync_handler(
    cause: usize,
    epc: usize,
    tval: usize,
) -> usize {
    let mut return_pc = epc;

    match cause {
        2 => {
            // Illegal instruction
            panic!("Illegal instruction -> {:#x}: {:#x}", epc, tval);
        },
        8 => {
            // Environment (system) call from User mode
            println!("E-call from User mode -> {:#x}", epc);
            return_pc += 4;
        },
        9 => {
            // Environment (system) call from Supervisor mode
            println!("E-call from Supervisor mode -> {:#x}", epc);
            return_pc += 4;
        },
        11 => {
            // Environment (system) call from Machine mode
            panic!("E-call from Machine mode -> {:#x}", epc);
        },
        // Page faults
        12 => {
            // Instruction page fault
            println!("Instruction page fault -> {:#x}: {:#x}", epc, tval);
            return_pc += 4;
        },
        13 => {
            // Load page fault
            println!("Load page fault -> {:#x}: {:#x}", epc, tval);
            return_pc += 4;
        },
        15 => {
            // Store page fault
            println!("Store page fault -> {:#x}: {:#x}", epc, tval);
            return_pc += 4;
        },
        _ => {
            panic!("Unhandled sync trap -> {}", cause);
        }
    }

    return_pc
}