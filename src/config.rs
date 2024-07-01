//! Configuration module
//! This module contains functions to configure the system timer (SysTick) and other peripherals

use cortex_m::peripheral::syst::SystClkSource;

/// Configures the system timer (SysTick), handler function is `embed_rs::exception::SysTick`
///
pub fn sys_tick_config() {
    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);
    // this is configured for the LM3S6965 which has a default CPU clock of 4 MHz
    syst.set_reload(4_000_000);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();
}
