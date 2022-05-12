use embedded_hal::adc::OneShot;
use embedded_hal::digital::v2::InputPin;
use nrf52832_hal::gpio::{p0, Pin, Input, Floating};
use nrf52832_hal::saadc::{Saadc};

pub struct BatteryStatus {
    /// Pin High = battery, Low = charging.
    pin_charge_indication: Pin<Input<Floating>>,

    /// Pin Voltage level
    pin_voltage: p0::P0_31<Input<Floating>>,

    /// SAADC peripheral
    saadc: Saadc,

    /// Charging state
    charging: bool,

    millivolts: i16,
}

impl BatteryStatus {
    /// Initialize the battery status.
    pub(super) fn init(
        saadc: Saadc,
        pin_charge_indication: Pin<Input<Floating>>,
        pin_voltage: p0::P0_31<Input<Floating>>,
    ) -> Self {
        let mut myself = Self {
            pin_charge_indication,
            pin_voltage,
            saadc,
            charging: false,
            millivolts: 0,
        };

        // gather the current state
        myself.update();
        myself
    }

    /// This returns the stored value. To fetch current data, call `update()` first.
    pub fn is_charging(&self) -> bool {
        self.charging
    }

    /// This returns the stored value. To fetch current data, call `update()` first.
    pub fn millivolts(&self) -> i16 {
        self.millivolts
    }

    /// Update the current battery status by reading information from the
    /// hardware. Return whether or not the values changed.
    pub fn update(&mut self) -> bool {
        let mut changed = false;

        // Check charging status
        let charging = self.pin_charge_indication.is_low().unwrap();
        if charging != self.charging {
            self.charging = charging;
            changed = true;
        }

        // Check voltage
        let millivolts = self.read_millivolts();
        if millivolts != self.millivolts {
            self.millivolts = millivolts;
            changed = true;
        }

        changed
    }

    /// Convert a raw ADC measurement into a battery voltage in 0.1 volts.
    fn read_millivolts(&mut self) -> i16 {
        let adc_val = self.saadc.read(&mut self.pin_voltage).unwrap();
        (adc_val * 2000) / (4095.0 / 3.3) as i16
    }
}