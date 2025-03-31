#![no_std]
#![no_main]

use stm32f4xx_hal::{self as hal, gpio::{gpioe, GpioExt}};


pub trait LED {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
}
pub struct AmberLed {
    pin: gpioe::PE12<hal::gpio::Output<hal::gpio::OpenDrain>>,
}

impl LED for AmberLed {

    fn turn_off(&mut self) {
        self.pin.set_low()
    }
    
    fn turn_on(&mut self) {
        self.pin.set_high()
    }
}

impl AmberLed {
    pub fn new(pin: gpioe::PE12<hal::gpio::Output<hal::gpio::OpenDrain>>) -> Self {
         AmberLed { pin }
    }

    pub fn default() -> Self {
         // Obtain device peripherals. This assumes that this is the only place where
         // `Peripherals::take()` is called.
         let dp = stm32f4xx_hal::pac::Peripherals::take().unwrap();
         // Split the GPIOE port into its independent pins.
         let gpioe = dp.GPIOE.split();
         // Convert PE12 to a push-pull output pin.
         let pin = gpioe.pe12.into_open_drain_output();
         AmberLed { pin }
    }
}


 