use cortex_m_rt::exception;
use cortex_m_semihosting::hprintln;

// by the sys_tick, we can read the operation system.
#[exception]
fn SysTick() {
    static mut COUNT: u32 = 0;

    // `COUNT` has transformed to type `&mut u32` and it's safe to use
    *COUNT += 1;
    hprintln!("SysTick: {}", *COUNT);
}

#[exception]
unsafe fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    hprintln!("ExceptionFrame {:#?}", ef);
    loop {}
}
