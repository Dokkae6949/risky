.section .text.init
.global _start
.type _start, @function

_start:
  # Set up the global pointer (GP) register.
  .option push
  .option norelax
    la gp, _global_pointer
  .option pop

    # Initialize the BSS section to zero.
    # This clears out the BSS section by setting it to zero.
    # BSS contains uninitialized static variables.
    la a0, _bss_start      # Load the start of BSS
    la a1, _bss_end        # Load the end of BSS
clear_bss:
    bgeu a0, a1, init_stack    # If start >= end, jump to stack initialization
    sd zero, (a0)              # Store zero into the BSS
    addi a0, a0, 8             # Move to the next 8-byte word
    j clear_bss                # Repeat until BSS is cleared

init_stack:
    # Set up the stack pointer.
    # The stack pointer is initialized to the end of the stack section.
    la sp, _stack_end

    # Call the Rust main function.
    # At this point, we assume that OpenSBI has set up the environment,
    # so we can directly call the main function.
    call main

halt:
    # If main returns, halt the system.
    # We don't expect to reach here, but just in case, we put the system
    # in an infinite loop.
    wfi                       # Wait for interrupt
    j halt                    # Jump back to halt and stay in the loop

# This is a dummy handler for other harts (cores).
park_hart:
    # Parked harts will just loop indefinitely.
    wfi                       # Wait for interrupt
    j park_hart               # Repeat indefinitely
