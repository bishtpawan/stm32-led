#![no_main]
#![no_std]

pub use panic_itm;
use cortex_m_rt::entry;
pub use stm32f3_discovery::stm32f3xx_hal;
use stm32f3xx_hal::prelude::*;
pub use stm32f3xx_hal::stm32;

#[entry]
fn main() -> ! {
    // initializing device peripherals to access led's pins
    let device_peripherals = stm32::Peripherals::take().unwrap();
    let mut rcc = device_peripherals.RCC.constrain();

    // initialize leds
    let mut gpioe = device_peripherals.GPIOE.split(&mut rcc.ahb);

    // configure LD3
    gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    // configure LD4
    gpioe.pe8.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    gpioe.pe10.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    gpioe.pe11.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    gpioe.pe12.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    loop {
        unsafe {
            // A magic address!
            const GPIOE_BSRR: u32 = 0x48001018;

            // Turn on LD3
            *(GPIOE_BSRR as *mut u32) = 1 << 9;

            // Turn on LD4
            *(GPIOE_BSRR as *mut u32) = 1 << 8;

            // Turn off LD3
            *(GPIOE_BSRR as *mut u32) = 1 << 25;

            // Turn off LD4
            *(GPIOE_BSRR as *mut u32) = 1 << 24;
        }
    }
}

