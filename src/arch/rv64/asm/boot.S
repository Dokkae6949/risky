.attribute arch, "rv64gc"
.option norvc

.section .text.init

.global _kentry
_kentry:
  # Set up the global pointer (GP) register.
  .option push
  .option norelax
    la gp, _global_pointer
  .option pop

    la sp, _stack_start
    li t0, 0x10000
    addi a0, a0, 1           # Increment the hart ID by 1
    mul t0, a0, t0           # Multiply the hart ID by 4096
    addi a0, a0, -1          # Decrement the hart ID by 1
    add sp, sp, t0           # Add the result to the stack pointer
    # Halt if the sp is greater than the stack end.
    # This should not happen unless:
    # - The stack size (t0) is not the same as the one in the linker script.
    # - The hart id is greater than the number supported amount defined in the linker script.
    la t0, _stack_end
    bgeu sp, t0, halt

    li t0, 0xc0ffee
    beq a1, t0, enter_kernel_ap

    la a1, _bss_start
    la a2, _bss_end
clear_bss:
    bgeu a1, a2, enter_kernel
    sd zero, (a1)
    addi a1, a1, 8
    j clear_bss

enter_kernel:
    call kentry

halt:
    # If main returns, halt the system.
    # We don't expect to reach here, but just in case, we put the system
    # in an infinite loop.
    wfi                       # Wait for interrupt
    j halt                    # Jump back to halt and stay in the loop

enter_kernel_ap:
    call kentry_ap

halt_ap:
    # If main returns, halt the system.
    # We don't expect to reach here, but just in case, we put the system
    # in an infinite loop.
    wfi                       # Wait for interrupt
    j halt_ap                 # Jump back to halt and stay in the loop