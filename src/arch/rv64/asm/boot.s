.attribute arch, "rv64gc"
.option norvc

.section .init, "ax"
.global _start
_start:
  .option push
  .option norelax # Prevent an unsupported R_RISCV_ALIGN relocation from being generated
relocate_entry:
    auipc ra, %pcrel_hi(relocate_target)
    ld ra, %pcrel_lo(relocate_entry)(ra)
    jr ra
  .align 3
relocate_target:
  .dword _abs_start
  .option pop

_abs_start:

  /* Setup the global pointer (GP) register */
  .option norelax # Prevent an unsupported R_RISCV_ALIGN relocation from being generated
  .cfi_startproc
  .cfi_undefined ra

  /* Disable interrupts */
  csrw sie, 0
  csrw sip, 0

  /* Setup pre-init trap vector */
  la t0, _pre_init_trap
  csrw stvec, t0

  /* Zero general-purpose registers */
  li x1, 0
  li x2, 0
  li x3, 0
  li x4, 0
  li x5, 0
  li x6, 0
  li x7, 0
  li x8, 0
  li x9, 0
  li x13, 0
  li x14, 0
  li x15, 0
  li x16, 0
  li x17, 0
  li x18, 0
  li x19, 0
  li x20, 0
  li x21, 0
  li x22, 0
  li x23, 0
  li x24, 0
  li x25, 0
  li x26, 0
  li x27, 0
  li x28, 0
  li x29, 0
  li x30, 0
  li x31, 0

  /* Setup the global pointer (GP) register */
  .option push
  .option norelax # Prevent an unsupported R_RISCV_ALIGN relocation from being generated
    la gp, __global_pointer$
  .option pop

    la sp, _stack_start
    mv t2, a0 # The HartID is passed by OpenSBI via the a0 reg

    lui t0, %hi(_max_hart_id)
    add t0, t0, %lo(_max_hart_id)

    bgtu t2, t0, abort # Abort if the hartid is bigger than the _max_hart_id

    lui t0, %hi(_hart_stack_size)
    add t0, t0, %lo(_hart_stack_size)

    /* Multiply the hartid and the _hart_stack_size to get the memory offset
       to the stack start of that hart */
    mul t0, t2, t0

    la t1, _stack_start
    sub t1, t1, t0 # Calculate the hart local stack start
    andi sp, t1, -16 # Align stack to 16-bytes
    add s0, sp, zero # Save the stack pointer to the frame pointer?

    /* Store A0..A2 on the stack for later use by the kernel main */
    addi sp, sp, -32 # Reserve stack space
    sd a0, 0(sp)
    sd a1, 8(sp)
    sd a2, 16(sp)
    # Followed by unused stack space for 16-byte alignment

    call main_mp
    mv t0, a0
    beqz a0, ram_initialized

    # Copy .data from flash to RAM if using flash
    # https://github.com/rust-embedded/riscv/blob/master/riscv-rt/src/asm.rs#L147

    la t0, _sbss
    la t2, _ebss
    bgeu t0, t2, ram_initialized
zero_bss_loop: # Write zero to every address
    sd zero, 0(t0)
    addi t0, t0, 8
    bltu t0, t2, zero_bss_loop
    
ram_initialized: # RAM initialized

    # Initialize floating point unit if present
    # https://github.com/rust-embedded/riscv/blob/master/riscv-rt/src/asm.rs#L187

    # Setup interrupts
    # https://github.com/rust-embedded/riscv/blob/master/riscv-rt/src/asm.rs#L187

    # Restore A0..A2 registers
    ld a0, 0(sp)
    ld a1, 8(sp)
    ld a2, 16(sp)
    addi sp, sp, 32 # Return the reserved stack space
    jal zero, main # Call main and ignore any return address
  .cfi_endproc
    
/* Default Multiprocessor Hook. Returns 1 if the hartid is
   0 otherwise busy-loops all other harts. */
.weak main_mp
main_mp:
    beqz a0, 2f # If hartid is 0 return 1 or "true"
1: wfi # Park hart in a loop
    j 1b
2: li a0, 1
    ret

/* Default implementation of `_pre_init_trap` is an infinite loop
   If the execution reaches this point, it means that there is a
   bug in the boot code. */
.section .init.trap, "ax"
.weak _pre_init_trap
_pre_init_trap:
  j _pre_init_trap

/* Just loop infinitely */
.section .text.abort
.weak abort
abort:
    j abort
