//! Exception handling
//! This module contains the exception handlers for the system timer (SysTick) and the hard fault handler
//! 
//! The exception that trigger by the hardware, the `interrupt` is mean trigger by the software. 

use cortex_m_rt::exception;

// by the sys_tick, we can read the operation system.
#[exception]
fn SysTick() {
    static mut COUNT: u32 = 0;

    // `COUNT` has transformed to type `&mut u32` and it's safe to use
    *COUNT += 1;
}

#[exception]
unsafe fn HardFault(_ef: &cortex_m_rt::ExceptionFrame) -> ! {
    // nothing to do here
    loop {}
}
