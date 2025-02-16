#![no_std]
#![no_main]

mod bsp;
mod console;
mod cpu;
mod driver;
mod panic_wait;
mod print;
mod synchronization;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    if let Err(e) = bsp::driver::init() {
        panic!("Error initializing BSP driver subsystem: {}", e)
    }
    
    driver::driver_manager().init_drivers();
    
    kernel_main();
}

fn kernel_main() -> ! {
    use console::console;
    
    println!(
        "[0] {} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("[1] Booting on: {}", bsp::board_name());

    println!("[2] Drivers loaded:");
    driver::driver_manager().enumerate();

    println!("[3] Chars written: {}", console().chars_written());
    println!("[4] Echoing input now");

    console().clear_rx();
    loop {
        let c = console().read_char();
        console().write_char(c);
    }
}
