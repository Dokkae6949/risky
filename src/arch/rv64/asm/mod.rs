use core::arch::asm;

/// Initialize the stack pointer
/// # Safety
/// This function is unsafe because it can cause undefined behavior
/// if the `a0` register is not set to the number of hart.
#[inline(always)]
pub unsafe fn init_stack_pointer() {
    asm!(
        "la sp, _stack_start",
        "li t0, 4096",
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