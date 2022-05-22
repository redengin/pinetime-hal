use nrf52832_hal::gpio::{Pin, Output, PushPull};
use nrf52832_hal::{prelude::OutputPin};

pub struct Vibrator {
    pub(super) pin: Pin<Output<PushPull>>,
}

impl Vibrator {
    pub fn on(&mut self) {
        self.pin.set_low().unwrap();
    }

    pub fn off(&mut self) {
        self.pin.set_high().unwrap();
    }
}