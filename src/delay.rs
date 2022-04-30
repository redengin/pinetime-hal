use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use nrf52832_hal::{
    pac,
    timer::Timer,
};

/// use TIMER0 as RTIC takes ownership of SYST
pub struct Delay {
    timer: nrf52832_hal::Timer<pac::TIMER0>,
}

impl Delay {
    pub fn init(timer0: pac::TIMER0) -> Self {
        Self {
            timer: Timer::new(timer0),
        }
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        // Currently the timer is hardcoded at 1 MHz, so 1 cycle = 1 µs.
        let cycles = us;
        self.timer.delay(cycles);
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        DelayUs::delay_us(self, ms * 1000);
    }
}
