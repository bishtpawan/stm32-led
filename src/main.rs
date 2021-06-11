#![no_main]
#![no_std]

pub use panic_itm;
use cortex_m_rt::entry;
use stm32::{LedArray, Delay, OutputSwitch, init};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = init();

    //let half_period = 500_u16;

    loop {
        leds[1].on().ok();
        leds[3].on().ok();
        leds[5].on().ok();
        leds[7].on().ok();
        cortex_m::asm::delay(2000000);

        leds[1].off().ok();
        leds[3].off().ok();
        leds[5].off().ok();
        leds[7].off().ok();
        cortex_m::asm::delay(2000000);

    }
}

