use core::arch::asm;
use crate::arch::consts::get_page_align;
use crate::arch::paging_sv39::Table;

/// Initialize the stack pointer
/// # Safety
/// This function is unsafe because it can cause undefined behavior
/// if the `a0` register is not set to the number of hart.
/// This function should be kept in sync with the boot.S file.
#[inline(always)]
pub unsafe fn init_stack_pointer() {
    asm!(
        "la sp, _stack_start",
        "li t0, 0x10000", // 64KB
        "addi a0, a0, 1",
        "mul t0, a0, t0",
        "addi a0, a0, -1",
        "add sp, sp, t0",
        options(nomem, nostack),
    );
}

/// Get the hart id of the current hart.
/// # Safety
/// This function is unsafe because it can cause undefined behavior
/// if the `a0` register is not set to the number of hart.
/// Usually, this function is called in the entry point of the kernel.
#[inline(always)]
pub unsafe fn get_hart_id() -> usize {
    let hart_id: usize;
    asm!("mv {}, a0", out(reg) hart_id);
    hart_id
}

/// Read the value of the `satp` register.
#[inline(always)]
pub fn read_satp() -> usize {
    let satp: usize;
    unsafe {
        asm!("csrr {}, satp", out(reg) satp);
    }
    satp
}

/// Write a value to the `satp` register.
#[inline(always)]
pub fn write_satp(satp: usize) {
    unsafe {
        asm!("csrw satp, {}", in(reg) satp);
    }
}

/// Check if virtual memory is enabled.
#[inline(always)]
pub fn is_virtual_memory_enabled() -> bool {
    let satp = read_satp();
    satp != 0 // If satp is non-zero, VM is enabled
}

#[inline(always)]
pub fn enable_virtual_memory_sv39(table: *mut Table) {
    write_satp((table as usize >> 12) | (8 << 60));
}

#[inline(always)]
pub fn get_time() -> u64 {
    let time: u64;
    unsafe {
        asm!("rdtime {}", out(reg) time);
    }
    time
}