pub mod console;
pub mod cpu;
pub mod driver;
pub mod memory;

pub fn board_name() -> &'static str {
    #[cfg(feature = "bsp_rpi3")]
    {
        "Raspberry Pi 3"
    }
}
