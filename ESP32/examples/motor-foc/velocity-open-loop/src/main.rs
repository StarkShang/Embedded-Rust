#![no_std]
#![no_main]

mod motor_control;

use esp_backtrace as _;
use esp_hal::{
    clock::{ClockControl, Clocks}, delay::Delay, gpio::Io, mcpwm::{operator::PwmPinConfig, timer::PwmWorkingMode, McPwm, PeripheralClockConfig}, peripherals::Peripherals, prelude::*, system::SystemControl
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals: Peripherals = Peripherals::take();
    let system: SystemControl = SystemControl::new(peripherals.SYSTEM);
    let clocks: Clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    // initialize peripheral
    let clock_cfg = PeripheralClockConfig::with_frequency(&clocks, 40.MHz()).unwrap();
    let mut mcpwm = McPwm::new(peripherals.MCPWM0, clock_cfg);
    mcpwm.operator0.set_timer(&mcpwm.timer0);
    mcpwm.operator1.set_timer(&mcpwm.timer1);
    mcpwm.operator2.set_timer(&mcpwm.timer2);
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let pin32 = io.pins.gpio32;
    let pin33 = io.pins.gpio33;
    let pin25 = io.pins.gpio25;
    //let pin12 = io.pins.gpio12;
    
    // connect operator0 to pin
    //let (mut pwm_pin_a, mut led_pin) = mcpwm.operator0.with_pins(pin32, PwmPinConfig::UP_ACTIVE_HIGH, pin12, PwmPinConfig::UP_ACTIVE_HIGH);
    let mut pwm_pin_a = mcpwm.operator0.with_pin_a(pin32, PwmPinConfig::UP_ACTIVE_HIGH);
    let mut pwm_pin_b = mcpwm.operator1.with_pin_a(pin33, PwmPinConfig::UP_ACTIVE_HIGH);
    let mut pwm_pin_c = mcpwm.operator2.with_pin_a(pin25, PwmPinConfig::UP_ACTIVE_HIGH);
    // start timer with timestamp values in the range of 0..=99 and a frequency of 20 kHz
    let timer_clock_cfg = clock_cfg
        .timer_clock_with_frequency(99, PwmWorkingMode::Increase, 20.kHz())
        .unwrap();
    mcpwm.timer0.start(timer_clock_cfg);
    mcpwm.timer1.start(timer_clock_cfg);
    mcpwm.timer2.start(timer_clock_cfg);

    let delay = Delay::new(&clocks);
    loop {
        let (d_a, d_b, d_c) = motor_control::velocity_openloop(10.0);
        let u_a = (d_a * 100.0) as u16;
        let u_b = (d_b * 100.0) as u16;
        let u_c = (d_c * 100.0) as u16;
        println!("d_a: {d_a}, d_b: {d_b}, d_c: {d_c}");
        println!("u_a: {u_a}, u_b: {u_b}, u_c: {u_c}");
        // pwm_pin_a.set_timestamp(u_a);
        // pwm_pin_b.set_timestamp(u_b);
        // pwm_pin_c.set_timestamp(u_c);
        //led_pin.set_timestamp(u_c);
        delay.delay_millis(200);
    }
}