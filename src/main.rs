#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use stm32l4::stm32l4x6;

#[entry]
fn main() -> ! {
    let p = stm32l4x6::Peripherals::take().unwrap();
    let rcc = p.RCC;
    let gpioa = p.GPIOA;
    let uart = p.USART1;

    // Enable clock for GPIOA and USART1
    rcc.ahb2enr.write(|w| w.gpioaen().set_bit());
    rcc.apb2enr.write(|w| w.usart1en().set_bit());

    // Configure PA9 as USART1_TX, PA10 as USART1_RX (AF7)
    gpioa
        .moder
        .modify(|_, w| w.moder9().alternate().moder10().alternate());
    gpioa.afrh.modify(|_, w| w.afrh9().af7().afrh10().af7());
    gpioa
        .ospeedr
        .modify(|_, w| w.ospeedr9().very_high_speed().ospeedr10().very_high_speed());
    gpioa
        .pupdr
        .modify(|_, w| w.pupdr9().pull_up().pupdr10().pull_up());

    // Configure USART1: 115200 baud, 8 data bits, no parity, 1 stop bit
    // Assuming a clock of 4MHz for USART1
    uart.brr.write(|w| w.brr().bits(35)); // Rounded BRR = Clock / Baud = 4_000_000 / 115200

    uart.cr1.write(|w| {
        w.ue()
            .set_bit() // USART enable
            .re()
            .set_bit() // Receiver enable
            .te()
            .set_bit() // Transmitter enable
            .m0()
            .clear_bit() // 1 Start bit, 8 Data bits, n Stop bit
            .m1()
            .clear_bit()
            .pce()
            .clear_bit() // Parity control disable
    });

    uart.cr2.write(|w| w.stop().bits(0)); // 1 stop bit

    // Assuming `uart` is your USART1 peripheral instance
    let msg = b"Hello, World! xjh!!\r\n";

    loop {
        for &byte in msg {
            // Wait for the transmit data register to be empty
            while uart.isr.read().txe().bit_is_clear() {}

            // Write the byte to the transmit data register
            uart.tdr.write(|w| w.tdr().bits(u16::from(byte)));
        }

        // Simple delay loop (not accurate, for demonstration only)
        // Adjust the count based on your system's clock speed for approximately 1 second
        for _ in 0..4_000 {
            cortex_m::asm::nop(); // No Operation (does nothing but waste time)
        }
    }
}
