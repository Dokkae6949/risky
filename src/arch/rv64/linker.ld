OUTPUT_ARCH("riscv")

ENTRY(main)

MEMORY {
  RAM (wxa) : ORIGIN = 0x80200000, LENGTH = 128M
}

REGION_ALIAS("REGION_TEXT", RAM);
REGION_ALIAS("REGION_RODATA", RAM);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

PROVIDE(_stack_start = ORIGIN(REGION_STACK) + LENGTH(REGION_STACK));
PROVIDE(_max_hart_id = 3);
PROVIDE(_hart_stack_size = 2K);
PROVIDE(_heap_size = 1M);

SECTIONS {
  /* Code Section */
  .text : {
    PROVIDE(_stext = .);
    
    /* Insert the the initial startup code first */
    KEEP(*(.init));
    . = ALIGN(4);
    KEEP(*(.init.trap));
    . = ALIGN(4);

    *(.text.abort);
    *(.text .text.*); /* Anything else goes here */

    . = ALIGN(4);
    PROVIDE(_etext = .);
  } > REGION_TEXT

  /* Read Only Data Section */
  .rodata : ALIGN(4) {
    . = ALIGN(4);
    PROVIDE(_srodata = .);

    *(.rodata .rodata.*);
    
    . = ALIGN(8);
    PROVIDE(_erodata = .);
  } > REGION_RODATA

  /* Data Section */
  .data : ALIGN(8) {
    . = ALIGN(8);
    PROVIDE(_sdata = .);

    /* Required for linker-relaxation to work */
    PROVIDE(__global_pointer$ = . + 0x800);

    *(.data .data.*);

    . = ALIGN(8);
    PROVIDE(_edata = .);
  } > REGION_DATA AT > REGION_RODATA

  /* LMA of the data section */
  PROVIDE(_sidata = LOADADDR(.data));

  /* BSS Section: Used for zero initialized variables */
  .bss (NOLOAD) : ALIGN(8) {
    . = ALIGN(8);
    PROVIDE(_sbss = .);

    *(.sbss .sbss.* .bss .bss.*);
    
    . = ALIGN(8);
    PROVIDE(_ebss = .);
  } > REGION_BSS

  /* Custom Section: Represents memory used for the heap */
  .heap (NOLOAD) : ALIGN(4) {
    PROVIDE(_sheap = .);
    
    . += _heap_size; /* TODO: Figure out heap size */

    . = ALIGN(4);
    PROVIDE(_eheap = .);
  } > REGION_HEAP

  /* Custom Section: Represents memory used for the stack */
  .stack (NOLOAD) : {
    PROVIDE(_estack = .);

    . = ABSOLUTE(_stack_start);

    PROVIDE(_sstack = .);
  } > REGION_STACK

  /* Fake Output .got Section */
  /* Dynamic relocations are unsupported. This section is only used to detect
     relocatable code in the input files and raise an error if relocatable code
     is found */
  .got (INFO) : {
    KEEP(*(.got .got.*));
  }
}

ASSERT(ORIGIN(REGION_TEXT) % 4 == 0, "
ERROR(rv64): the start of the REGION_TEXT must be 4-byte aligned");

ASSERT(ORIGIN(REGION_RODATA) % 4 == 0, "
ERROR(rv64): the start of the REGION_RODATA must be 4-byte aligned");

ASSERT(ORIGIN(REGION_DATA) % 8 == 0, "
ERROR(rv64): the start of the REGION_DATA must be 8-byte aligned");

ASSERT(ORIGIN(REGION_HEAP) % 4 == 0, "
ERROR(rv64): the start of the REGION_HEAP must be 4-byte aligned");

ASSERT(ORIGIN(REGION_STACK) % 4 == 0, "
ERROR(rv64): the start of the REGION_STACK must be 4-byte aligned");

ASSERT(_stext % 4 == 0, "
ERROR(rv64): `_stext` must be 4-byte aligned");

ASSERT(_sdata % 8 == 0 && _edata % 8 == 0, "
BUG(rv64): .data is not 8-byte aligned");

ASSERT(_sidata % 8 == 0, "
BUG(rv64): the LMA of .data is not 8-byte aligned");

ASSERT(_sbss % 8 == 0 && _ebss % 8 == 0, "
BUG(rv64): .bss is not 8-byte aligned");

ASSERT(_sheap % 4 == 0, "
BUG(rv64): start of .heap is not 4-byte aligned");

ASSERT(_stext + SIZEOF(.text) < ORIGIN(REGION_TEXT) + LENGTH(REGION_TEXT), "
ERROR(rv64): The .text section must be placed inside the REGION_TEXT region.
Set _stext to an address smaller than 'ORIGIN(REGION_TEXT) + LENGTH(REGION_TEXT)'");

ASSERT(SIZEOF(.stack) > (_max_hart_id + 1) * _hart_stack_size, "
ERROR(rv64): .stack section is too small for allocating stacks for all the harts.
Consider changing `_max_hart_id` or `_hart_stack_size`.");

ASSERT(SIZEOF(.got) == 0, "
ERROR(rv64): .got section detected in the input files. Dynamic relocations are not
supported. If you are linking to C code compiled using the `cc` crate then modify your
build script to compile the C code _without_ the -fPIC flag. See the documentation of
the `cc::Build.pic` method for details.");
