use nrf52832_hal::{gpio::{Pin, Output, PushPull}, prelude::OutputPin};

pub struct Vibrator {
    pin: Pin<Output<PushPull>>,
}

impl Vibrator {
    pub (crate) fn new(
        pin: Pin<Output<PushPull>>,
    ) -> Self {
        Self { pin }
    }

    pub fn on(&mut self) {
        self.pin.set_high().unwrap();
    }

    pub fn off(&mut self) {
        self.pin.set_low().unwrap();
    }
}