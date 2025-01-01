#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    init,
    mcpwm::{operator::PwmPinConfig, timer::PwmWorkingMode, McPwm, PeripheralClockConfig},
    prelude::*,
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    // Set GPIO12 as an output, and set its state high initially.
    let led = Output::new(peripherals.GPIO12, Level::Low);

    // initialize peripheral
    let clock_cfg = PeripheralClockConfig::with_frequency(32.MHz()).unwrap();
    let mut mcpwm = McPwm::new(peripherals.MCPWM0, clock_cfg);
    // connect operator0 to timer0
    mcpwm.operator0.set_timer(&mcpwm.timer0);
    // connect operator0 to pin
    let mut pwm_pin = mcpwm
        .operator0
        .with_pin_a(led, PwmPinConfig::UP_ACTIVE_HIGH);

    // start timer with timestamp values in the range of 0..=99 and a frequency of 20 kHz
    let timer_clock_cfg = clock_cfg
        .timer_clock_with_frequency(99, PwmWorkingMode::Increase, 20.kHz())
        .unwrap();
    mcpwm.timer0.start(timer_clock_cfg);

    let delay = Delay::new();
    let mut brightness = 0u16;
    let mut increase_or_decrease = true;
    loop {
        println!("Current Brightness: {brightness}");
        pwm_pin.set_timestamp(brightness);
        if brightness > 99 {
            increase_or_decrease = false;
        } else if brightness < 1 {
            increase_or_decrease = true;
            delay.delay_millis(500u32);
        }

        if increase_or_decrease {
            brightness += 1;
        } else {
            brightness -= 1;
        }

        delay.delay_millis(10u32);
    }
}
