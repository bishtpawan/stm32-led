#![no_std]

pub use stm32f3_discovery::stm32f3xx_hal;
use stm32f3xx_hal::prelude::*;
pub use stm32f3xx_hal::stm32;

pub fn init() {

    // initializing device peripherals to access led's pins
    let device_peripherals = stm32::Peripherals::take().unwrap();
    let mut rcc = device_peripherals.RCC.constrain();

    // initialize leds
    let mut gpioe = device_peripherals.GPIOE.split(&mut rcc.ahb);

    // configure led 3
    gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    // configure led 10
    gpioe.pe13.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
}


