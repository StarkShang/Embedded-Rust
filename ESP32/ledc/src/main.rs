#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::Io,
    ledc::{channel, timer, LSGlobalClkSource, Ledc, LowSpeed},
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // 配置输出引脚
    // 本例程 Pin12 连接至 LED
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let pin12 = io.pins.gpio12;
    // 配置 LEDC 外设
    let mut ledc = Ledc::new(peripherals.LEDC, &clocks);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    let mut lstimer0 = ledc.get_timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0.configure(timer::config::Config {
        duty: timer::config::Duty::Duty5Bit,
        clock_source: timer::LSClockSource::APBClk,
        frequency: 24.kHz(),
    }).unwrap();

    let mut channel0 = ledc.get_channel(channel::Number::Channel0, pin12);
    


    let mut brightness = 0;
    let mut increase_or_decrease = true;
    let delay = Delay::new(&clocks);
    loop {
        println!("Current Brightness: {brightness}");
        let _ = channel0.configure(channel::config::Config{
            timer: &lstimer0,
            duty_pct: brightness,
            pin_config: channel::config::PinConfig::PushPull
        });
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