KERNEL_ENTRYPOINT = 0x80000;
DRAM_START = 0;

ENTRY(KERNEL_ENTRYPOINT)

/*
 *     4 == R
 *     5 == RX
 *     6 == RW
 */
PHDRS
{
    segment_boot_core_stack PT_LOAD FLAGS(6);
    segment_code PT_LOAD FLAGS(5);
    segment_data PT_LOAD FLAGS(6);
}

SECTIONS
{
    . =  DRAM_START;
    
    .boot_core_stack (NOLOAD) :
    {
        . += KERNEL_ENTRYPOINT;
        __boot_core_stack_end_exclusive = .;
    } :segment_boot_core_stack

    .text :
    {
        KEEP(*(.text._start))
        *(.text._start_arguments)
        *(.text._start_rust)
        *(.text*)
    } :segment_code
    
    .rodata : ALIGN(8)
    {
        *(.rodata*)
    } :segment_code
    
    .data :
    {
        *(.data*)
    } :segment_data
    
    .bss : ALIGN(16)
    {
        __bss_start = .;
        *(.bss*)
        . = ALIGN(16);
        __bss_end_exclusive = .;
    } :segment_data
    
    .got : { *(.got*) }
    ASSERT(SIZEOF(.got) == 0, "Relocation support not expected")

    /DISCARD/ : { *(.comment*) }
}
