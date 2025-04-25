.section .text._start

.macro ADR_REL register, symbol
    auipc    \register, %pcrel_hi(\symbol)
    add     \register, \register, %pcrel_lo(\symbol)
.endm

_start:
    # read hart (core)
    csrr a0, mhartid
    la a1, BOOT_CORE_ID
    ld a1, (a1)
    bne a0, a1, .L_parking_loop
    
    ADR_REL a0, __bss_start
    ADR_REL a1, __bss_end_exclusive

.L_bss_init_loop:
    beq a0, a1, .L_prepare_rust
    sd zero, (a0)
    addi a0, a0, 8
    j .L_bss_init_loop

.L_prepare_rust:
    ADR_REL	sp, __boot_core_stack_end_exclusive
    
    ADR_REL a1, ARCH_TIMER_COUNTER_FREQUENCY
    csrr a2, mcycle
    beqz a2, .L_parking_loop
    sw a2, (a1)
    
    j _start_rust
    
.L_parking_loop:
    wfi
    j .L_parking_loop

.size _start, . - _start
.type _start, function
.global _start
