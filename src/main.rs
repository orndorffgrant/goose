#![no_std]
#![no_main]

mod bsp;
mod console;
mod cpu;
mod driver;
mod panic_wait;
mod print;
mod synchronization;
mod time;

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
    use core::time::Duration;
    
    info!(
        "{} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    info!("Booting on: {}", bsp::board_name());

    info!("Drivers loaded:");
    driver::driver_manager().enumerate();
    
    info!("Testing timer");
    time::time_manager().spin_for(Duration::from_nanos(1));
    info!("Spinning for 1 second");
    time::time_manager().spin_for(Duration::from_secs(1));
    info!("Spinning for 1 second");
    time::time_manager().spin_for(Duration::from_secs(1));
    info!("Spinning for 1 second");
    time::time_manager().spin_for(Duration::from_secs(1));
    info!("Spinning for 1 second");
    time::time_manager().spin_for(Duration::from_secs(1));
    
    info!("Chars written: {}", console().chars_written());
    
    info!("Echoing input now");

    console().clear_rx();
    loop {
        let c = console().read_char();
        console().write_char(c);
    }
}
