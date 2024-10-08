//! # STM32L4 HAL
//! 
//! this project is learn the rust on embed, the used board is STM32L496.
//! 

#![no_std]
#![no_main]

// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support

mod exception;
mod config;

use config::sys_tick_config;
// the semihosting need to change on the production version
// use cortex_m_semihosting::hprintln;
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use stm32l4::stm32l4x6;

#[entry]
fn main() -> ! {
    sys_tick_config();

    let p = stm32l4x6::Peripherals::take().unwrap();
    let rcc = p.RCC;

    // Assuming `uart` is your USART1 peripheral instance
    let _msg = "Hello, World! xjh!!\r\n";

    // use the PE9 pin as a led light to show the program is running, blue light
    let gpioe = p.GPIOE;
    rcc.ahb2enr.write(|w| w.gpioeen().set_bit());
    gpioe.moder.write(|w| w.moder9().output());
    gpioe.odr.write(|w| w.odr9().set_bit());

    loop {
        // Toggle the LED light
        gpioe.odr.modify(|r, w| w.odr9().bit(!r.odr9().bit()));
        // hprintln!("{}", msg);

        // Simple delay loop (not accurate, for demonstration only)
        // Adjust the count based on your system's clock speed for approximately 1 second
        for _ in 0..8_000 {
            cortex_m::asm::nop(); // No Operation (does nothing but waste time)
        }
    }
}
