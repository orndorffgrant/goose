use aarch64_cpu::asm;

pub use asm::nop;

#[inline(always)]
pub fn spin_for_cycles(n: usize) {
    for _ in 0..n {
        asm::nop();
    }
}

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe();
    }
}
