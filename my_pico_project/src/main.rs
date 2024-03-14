#![no_std]
#![no_main]

use pico_sdk::pico::hal::gpio::{self, GpioDirection};

// Entry point
#[no_mangle]
pub extern "C" fn main() -> ! {
    // Initialize GPIO 25 as an output
    gpio::init();
    gpio::set_direction(25, GpioDirection::Output);

    loop {
        // Toggle GPIO 25
        gpio::write(25, !gpio::read(25));
        
        // Delay for a short period to make the blinking visible
        for _ in 0..1_000_000 {
            cortex_m::asm::nop();
        }
    }
}
