/*
 * For some reason I can't get the linker script to load from
 * withing the arch/ dir so I'm just going to put it here for now.
 */

OUTPUT_ARCH("riscv")

ENTRY(_kentry)

MEMORY {
  ram (wxa) : ORIGIN = 0x80200000, LENGTH = 128M
}

PHDRS {
  text PT_LOAD;
  data PT_LOAD;
  bss PT_LOAD;
}

SECTIONS {
  .text : { /* code section */
    PROVIDE(_text_start = .);
    *(.text.init) /* anything in the .text.init section */
    *(.text .text.*) /* anything else in .text */
    PROVIDE(_text_end = .);
  } >ram AT>ram :text /* put this section into the text segment */

  /* this is magic, google "linker relaxation" */
  PROVIDE(_global_pointer = .);

  .rodata : { /* read only data section */
    PROVIDE(_rodata_start = .);
    *(.rodata .rodata.*)
    PROVIDE(_rodata_end = .);
  } >ram AT>ram :text

  .data : { /* data section */
    . = ALIGN(4096);
    PROVIDE(_data_start = .);
    *(.sdata .sdata.*) *(.data .data.*)
    PROVIDE(_data_end = .);
  } >ram AT>ram :data /* put this into the data segment */

  .bss : { /* bss section */
    PROVIDE(_bss_start = .);
    *(.sbss .sbss.*) *(.bss .bss.*)
    PROVIDE(_bss_end = .);
  } >ram AT>ram :bss /* put this section into the bss segment */

  PROVIDE(_memory_start = ORIGIN(ram));
  PROVIDE(_stack_start = _bss_end);
  PROVIDE(_stack_end = _stack_start + 4096 * 4);
  PROVIDE(_memory_end = ORIGIN(ram) + LENGTH(ram));

  PROVIDE(_heap_start = _stack_end);
  PROVIDE(_heap_size = _memory_end - _heap_start);
}
