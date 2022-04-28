//! use TIMER0 as RTIC takes ownership of SYST

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use nrf52832_hal::{pac, timer::Timer};

pub struct Delay {
    timer: nrf52832_hal::Timer<pac::TIMER0>,
}

impl Delay {
    pub fn new(timer0: pac::TIMER0) -> Self {
        Self {
            timer: Timer::new(timer0),
        }
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        // 1 cycle = 1 µs (per the nrf52832_hal configuration of TIMER0)
        let cycles = us;
        self.timer.delay(cycles);
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        DelayUs::delay_us(self, ms * 1000);
    }
}
