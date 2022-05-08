use embedded_hal::digital::v2::OutputPin;
use nrf52832_hal::gpio::{Output, Pin, PushPull};
#[cfg(test)]
use rtt_target::rprintln;

/// There are three active-low backlight pins, each connected to a FET that
/// toggles backlight power through a resistor.
/// Through combinations of these pins, 7 brightness levels (+ off) can be
/// configured.
pub struct Backlight {
    low: Pin<Output<PushPull>>,
    mid: Pin<Output<PushPull>>,
    high: Pin<Output<PushPull>>,
}

impl Backlight {
    /// Initialize the backlight with the specified level (0–7).
    pub(crate) fn init(
        low: Pin<Output<PushPull>>,
        mid: Pin<Output<PushPull>>,
        high: Pin<Output<PushPull>>,
    ) -> Self {
        Self {
            low,
            mid,
            high,
        }
    }

    /// Set the brightness level. Must be a value between 0 (off) and 7 (max
    /// brightness). Higher values are clamped to 7.
    pub fn set(&mut self, mut brightness: u8) {
        if brightness > 7 {
            brightness = 7;
        }
        #[cfg(test)]
        rprintln!("Setting backlight brightness to {}", brightness);

        if brightness & 0x01 > 0 {
            self.low.set_low().unwrap();
        } else {
            self.low.set_high().unwrap();
        }
        if brightness & 0x02 > 0 {
            self.mid.set_low().unwrap();
        } else {
            self.mid.set_high().unwrap();
        }
        if brightness & 0x04 > 0 {
            self.high.set_low().unwrap();
        } else {
            self.high.set_high().unwrap();
        }
    }
}
