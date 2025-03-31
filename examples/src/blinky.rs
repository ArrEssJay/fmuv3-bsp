//! Simple Rust firmware for Pixhawk 2 Cube (FMUv3)
//! Blinks an LED and beeps a buzzer periodically

#![deny(unsafe_code)]
#![no_main]
#![no_std]


use fmuv3_bsp::{AmberLed, LED};
use panic_halt as _;

use stm32f4xx_hal::{pac, prelude::*};
use cortex_m_rt::entry;
use cortex_m::delay::Delay;



#[entry]
fn main() -> ! {


    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();

    // CLOCKS

    let clocks = rcc
        .cfgr
        .use_hse(24_u32.MHz())  // Use 24 MHz external crystal
        .sysclk(168_u32.MHz())  // HCLK (AHB bus speed) at 168 MHz
        .pclk1(42_u32.MHz())    // APB1: 42 MHz (max 42 MHz for peripherals like USART, I2C)
        .pclk2(84_u32.MHz())    // APB2: 84 MHz (for SPI, ADC, etc.)
        .require_pll48clk() // Ensures correct 48 MHz clock for USB
        .freeze();

    let mut delay_source = Delay::new(cp.SYST, clocks.sysclk().to_Hz());
    
    let mut amber: AmberLed = AmberLed::default();
    
    let mut delay_ms: u32 = 1000;
    loop {

        amber.turn_on();
        
        delay_source.delay_ms(delay_ms);
        
        amber.turn_off();

        delay_source.delay_ms(delay_ms);
        if delay_ms > 50 { delay_ms = delay_ms - 50; }
        else {delay_ms = 1000}

    }
}
