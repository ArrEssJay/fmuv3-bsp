// Prints default clock speeds, should work for all STM32WL5x boards.

#![no_std]
#![no_main]

use panic_halt as _;

use stm32f4xx_hal::{pac, rcc::RccExt};
use cortex_m_rt::entry;
use cortex_m::asm;

use defmt;
use defmt_serial as _;

#[entry]
fn main() -> ! {

    
    let dp: pac::Peripherals = defmt::unwrap!(pac::Peripherals::take());
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    defmt::println!("sysclk_hz: {}", clocks.sysclk().to_Hz());
    defmt::println!("hclk_hz: {}", clocks.hclk().to_Hz());
    defmt::println!("pclk1_hz: {}", clocks.pclk1().to_Hz());
    defmt::println!("pclk2_hz: {}", clocks.pclk2().to_Hz());
    // Note: LSI frequency is not available through this HAL.

    loop {
        asm::bkpt();
    }
}