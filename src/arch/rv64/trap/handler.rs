use core::arch::asm;
use crate::arch::trap::{extract_scause, get_trap_cause, read_sepc, write_sepc};

#[no_mangle]
pub extern "riscv-interrupt-s" fn s_mode_trap_handler() {
    let (is_async, cause) = extract_scause(get_trap_cause());
    let cause = cause & 0xfff;
    let mut epc = read_sepc();
    let tval: usize;
    unsafe {asm!("csrr t0, mtval", out("t0") tval)};
    let hart: usize;
    unsafe {asm!("csrr t0, mhartid", out("t0") hart)};

    if is_async {
        epc = s_mode_asynchronous_handler(epc, tval, cause, hart)
    } else {
        epc = s_mode_synchronous_handler(epc, tval, cause, hart)
    }

    write_sepc(epc);
}

/// Asynchronous trap handler.
/// Returns the return program counter at which
/// the program should continue execution.
pub extern fn s_mode_asynchronous_handler(
    epc: usize,
    tval: usize,
    cause: usize,
    hart: usize,
) -> usize {
    let mut epc_return = epc;

    match cause {
        1 => {
            // Supervisor software interrupt
            println!("Supervisor software interrupt! CPU#{} -> {:#x}", hart, epc);
        }
        3 => {
            // Machine software interrupt
            println!("Machine software interrupt! CPU#{} -> {:#x}", hart, epc);
        }
        5 => unsafe {
            // Supervisor timer interrupt
            let stimecmp = 0x0200_0000 as *mut u64;
            let stime = 0x0200_0008 as *const u64;
            // The frequency given by QEMU is 10_000_000 Hz, so this sets
            // the next interrupt to fire one second from now.
            stimecmp.write_volatile(stime.read_volatile() + 10_000_000);
        }
        7 => unsafe {
            // Machine timer
            let mtimecmp = 0x0200_4000 as *mut u64;
            let mtime = 0x0200_bff8 as *const u64;
            // The frequency given by QEMU is 10_000_000 Hz, so this sets
            // the next interrupt to fire one second from now.
            mtimecmp.write_volatile(mtime.read_volatile() + 10_000_000);
        }
        9 => {
            // Supervisor external interrupt
            println!("Supervisor external interrupt! CPU#{} -> {:#x}", hart, epc);
        }
        11 => {
            // Machine external interrupt
            println!("Machine external interrupt! CPU#{} -> {:#x}", hart, epc);
        }
        _ => {
            panic!("Unhandled async trap! CPU#{} -> {}\n", hart, cause);
        }
    }

    epc_return
}

/// Synchronous trap handler.
/// Returns the return program counter at which
/// the program should continue execution.
pub fn s_mode_synchronous_handler(
    epc: usize,
    tval: usize,
    cause: usize,
    hart: usize,
) -> usize {
    let mut epc_return = epc;

    match cause {
        2 => {
            // Illegal instruction
            panic!("Illegal instruction! CPU#{} -> {:#x}: {:#x}", hart, epc, tval);
        }
        8 => {
            // Environment (system) call from User mode
            println!("E-call from User mode! CPU#{} -> {:#x}", hart, epc);
            epc_return += 4;
        }
        9 => {
            // Environment (system) call from Supervisor mode
            println!("E-call from Supervisor mode! CPU#{} -> {:#x}", hart, epc);
            epc_return += 4;
        }
        11 => {
            // Environment (system) call from Machine mode
            panic!("E-call from Machine mode! CPU#{} -> {:#x}", hart, epc);
        }
        // Page faults
        12 => {
            // Instruction page fault
            println!("Instruction page fault! CPU#{} -> {:#x}: {:#x}", hart, epc, tval);
            epc_return += 4;
        }
        13 => {
            // Load page fault
            println!("Load page fault! CPU#{} -> {:#x}: {:#x}", hart, epc, tval);
            epc_return += 4;
        }
        15 => {
            // Store page fault
            println!("Store page fault! CPU#{} -> {:#x}: {:#x}", hart, epc, tval);
            epc_return += 4;
        }
        _ => {
            panic!("Unhandled sync trap CPU#{} -> {}\n", hart, cause);
        }
    }

    epc_return
}