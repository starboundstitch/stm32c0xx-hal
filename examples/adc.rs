#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::Write;

use cortex_m_rt::entry;
use panic_halt as _;

use hal::analog::adc::{OversamplingRatio, Precision, SampleTime};
use hal::prelude::*;
use hal::stm32::{CorePeripherals, Peripherals};
use stm32c0xx_hal as hal;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut delay = cp.SYST.delay(&mut rcc);

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    // set up UART to output data
    let mut uart = dp
        .USART1
        .usart(
            (gpiob.pb6, gpiob.pb7),
            hal::serial::Config::default(),
            &mut rcc,
        )
        .unwrap();
    writeln!(uart, "\r\nSTM32C0 ADC Test\r\n").unwrap();

    // set up ADC with oversampling
    let mut adc = dp.ADC.constrain(&mut rcc);
    adc.set_sample_time(SampleTime::T_80);
    adc.set_precision(Precision::B_12);
    adc.set_oversampling_ratio(OversamplingRatio::X_16);
    //adc.set_oversampling_shift(4);
    adc.oversampling_enable(true);

    delay.delay(20.micros()); // Wait for ADC voltage regulator to stabilize before calibration
    adc.calibrate();

    let mut adc_pin = gpioa.pa4.into_analog();

    loop {
        let mv = adc.read_voltage(&mut adc_pin);
        write!(uart, "{mv:>5} mV\r\n").ok();

        delay.delay(1000.millis());
    }
}
