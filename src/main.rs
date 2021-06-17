#![no_main]
#![no_std]


pub use panic_itm;
use cortex_m_rt::entry;

/*
// Custom panic implementation
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}*/

#[entry]
fn main() -> ! {
    let _company_name = "KNOLDUS";
    let _practice_area = "RUST_EMBEDDED";
    let _device_name = "STM32F3DISCOVERY";

    //
    loop {}
}

