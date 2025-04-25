use crate::{cpu, println};
use core::panic::PanicInfo;


fn panic_prevent_reenter() {
    use core::sync::atomic::{AtomicBool, Ordering};

    // TODO actually check if this is fine for riscv64
    #[cfg(not(any(target_arch = "aarch64", target_arch = "riscv64")))]
    compile_error!("Add the target_arch to above's check if the following code is safe to use");

    static PANIC_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

    if !PANIC_IN_PROGRESS.load(Ordering::Relaxed) {
        PANIC_IN_PROGRESS.store(true, Ordering::Relaxed);

        return;
    }

    cpu::wait_forever()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    
    panic_prevent_reenter();
    
    let timestamp = crate::time::time_manager().uptime();
    let (location, line, column) = match info.location() {
        Some(location) => (location.file(), location.line(), location.column()),
        None => ("unknown", 0, 0),
    };
    
    println!(
        "[  {:>3}.{:06}] Kernel panic!\n  at: {}:{}:{}\n  message: {}",
        timestamp.as_secs(), timestamp.subsec_micros(),
        location, line, column, info.message()
    );
    
    cpu::wait_forever()
}
