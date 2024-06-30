#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::Io,
    mcpwm::{operator::PwmPinConfig, timer::PwmWorkingMode, McPwm, PeripheralClockConfig},
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Set GPIO12 as an output, and set its state high initially.
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let pin12 = io.pins.gpio12;

    // initialize peripheral
    let clock_cfg = PeripheralClockConfig::with_frequency(&clocks, 40.MHz()).unwrap();
    let mut mcpwm = McPwm::new(peripherals.MCPWM0, clock_cfg);
    // connect operator0 to timer0
    mcpwm.operator0.set_timer(&mcpwm.timer0);
    // connect operator0 to pin
    let mut pwm_pin = mcpwm
        .operator0
        .with_pin_a(pin12, PwmPinConfig::UP_ACTIVE_HIGH);
    
    // start timer with timestamp values in the range of 0..=99 and a frequency of 20 kHz
    let timer_clock_cfg = clock_cfg
        .timer_clock_with_frequency(99, PwmWorkingMode::Increase, 20.kHz())
        .unwrap();
    mcpwm.timer0.start(timer_clock_cfg);
    
    let delay = Delay::new(&clocks);
    let mut brightness = 0u16;
    let mut increase_or_decrease = true;
    loop {
        println!("Current Brightness: {brightness}");
        pwm_pin.set_timestamp(brightness);
        if brightness > 99 {
            increase_or_decrease = false;
        }
        else if brightness < 1 {
            increase_or_decrease = true;
            delay.delay_millis(500u32);
        }

        if increase_or_decrease {
            brightness += 1;
        }
        else {
            brightness -= 1;
        }

        delay.delay_millis(10u32);
    }
}