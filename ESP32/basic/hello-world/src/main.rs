#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{gpio::{Io, Level, Output}, peripherals::Peripherals, prelude::*};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();

    println!("Hello world!");

    // Set GPIO7 as an output, and set its state high initially.
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let _led = Output::new(io.pins.gpio7, Level::High);

    loop { }
}
