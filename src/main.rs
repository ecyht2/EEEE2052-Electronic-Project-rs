//! This example shows a complete project, including file structure, and config
//! needed to flash using an ST-Link. The project structure is based on
//! [Knurling's app-template](https://github.com/knurling-rs/app-template).
//! This file demonstrates an overview of this library's features.

//! See the syntax example in the main STM32-HAL repo for a more detailed example.

#![no_main]
#![no_std]

use cortex_m::{self, delay::Delay};
use cortex_m_rt::entry;

// These lines are part of our setup for debug printing.
use defmt_rtt as _;
use doppler_radar::LiquidCrystal;
use panic_probe as _;

// Import parts of this library we use. You could use this style, or perhaps import
// less here.
use stm32_hal2::{self, clocks::Clocks, gpio::{Pin, Port, PinMode}, pac};

#[entry]
fn main() -> ! {
    // Set up ARM Cortex-M peripherals. These are common to many MCUs, including all STM32 ones.
    let cp = cortex_m::Peripherals::take().unwrap();
    // Set up peripherals specific to the microcontroller you're using.
    let mut _dp = pac::Peripherals::take().unwrap();

    // This line is required to prevent the debugger from disconnecting on entering WFI.
    // This appears to be a limitation of many STM32 families. Not required in production code,
    // and significantly increases power consumption in low-power modes.
    stm32_hal2::debug_workaround();

    // Create an initial clock configuration that uses the MCU's internal oscillator (HSI),
    // sets the MCU to its maximum system clock speed.
    let clock_cfg = Clocks::default();

    // Write the clock configuration to the MCU. If you wish, you can modify `clocks` above
    // in accordance with [its docs](https://docs.rs/stm32-hal2/0.2.0/stm32_hal2/clocks/index.html),
    // and the `clock_cfg` example.
    clock_cfg.setup().unwrap();

    // Delay
    let mut delay = Delay::new(cp.SYST, clock_cfg.systick());

    // LCD
    let mut lcd = LiquidCrystal::new(
        Pin::new(Port::A, 8, PinMode::Output),
        Pin::new(Port::B, 10, PinMode::Output),
        Pin::new(Port::B, 4, PinMode::Output),
        Pin::new(Port::B, 5, PinMode::Output),
        Pin::new(Port::A, 9, PinMode::Output),
        Pin::new(Port::C, 7, PinMode::Output),
    );
    lcd.init(&mut delay);
    lcd.send_string("Hello World", &mut delay);

    loop {
        defmt::println!("Looping!"); // A print statement using DEFMT.
                                     // Enter a low power mode. The program will wake once an interrupt fires.
                                     // For example, the timer and GPIO interrupt above. But we haven't unmasked
                                     // their lines, so they won't work - see the `interrupts` example for that.
        delay.delay_ms(1000);
    }
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
