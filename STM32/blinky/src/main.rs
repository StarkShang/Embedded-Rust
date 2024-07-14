#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // 访问核心外设
    let cp = cortex_m::Peripherals::take().unwrap();
    // 访问设备制定外设
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);

    let mut delay = cp.SYST.delay(&clocks);
    loop {
        led.toggle();
        delay.delay_ms(500u16);
    }
}
