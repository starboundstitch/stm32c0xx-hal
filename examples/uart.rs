#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32c0xx_hal as hal;

use core::fmt::Write;

use hal::prelude::*;
use hal::serial::Config;
use hal::stm32;
use nb::block;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();
    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut usart = dp
        .USART1
        .usart((gpiob.pb6, gpiob.pb7), Config::default(), &mut rcc)
        .unwrap();

    writeln!(usart, "Hello\r").unwrap();

    let mut cnt = 0;
    loop {
        let byte = block!(usart.read()).unwrap_or(0);
        writeln!(usart, "{}: {}\r", cnt, byte).unwrap();
        cnt += 1;
    }
}
