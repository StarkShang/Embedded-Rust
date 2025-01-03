#![no_std]
#![no_main]

use core::cell::RefCell;
use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{Event, Gpio9, Input, PullUp, IO},
    interrupt,
    peripherals::Peripherals,
    prelude::*};
use esp_println::println;

static BUTTON: Mutex<RefCell<Option<Gpio9<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    println!("Hello world!");

    // Set GPIO7 as an output, and set its state high initially.
    let mut io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    io.set_interrupt_handler(handler);
    let mut led = io.pins.gpio7.into_push_pull_output();
    let mut button = io.pins.gpio9.into_pull_up_input();

    critical_section::with(|cs| {
        button.listen(Event::FallingEdge);
        BUTTON.borrow_ref_mut(cs).replace(button)
    });

    let delay = Delay::new(&clocks);
    loop {
        led.toggle();
        delay.delay_millis(500u32);
    }
}


#[handler]
fn handler() {
    critical_section::with(|cs| {
        println!("GPIO interrupt");
        BUTTON
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();
    });
}