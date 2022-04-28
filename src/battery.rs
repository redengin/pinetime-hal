use embedded_hal::adc::OneShot;
use embedded_hal::digital::v2::InputPin;
use nrf52832_hal::gpio::{p0, Floating, Input};
use nrf52832_hal::saadc::{Saadc, SaadcConfig};
use nrf52832_hal::target::SAADC;

pub struct BatteryStatus {
    /// Pin P0.12: High = battery, Low = charging.
    pin_charge_indication: p0::P0_12<Input<Floating>>,

    /// Pin P0.31: Voltage level
    pin_voltage: p0::P0_31<Input<Floating>>,

    /// SAADC peripheral
    saadc: Saadc,

    /// Charging state
    charging: bool,

    millivolts: i16,
}

impl BatteryStatus {
    /// Initialize the battery status.
    pub fn init(
        pin_charge_indication: p0::P0_12<Input<Floating>>,
        pin_voltage: p0::P0_31<Input<Floating>>,
        hw_saadc: SAADC,
    ) -> Self {
        let saadc = Saadc::new(hw_saadc, SaadcConfig::default());

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

    /// Return whether the watch is currently charging.
    ///
    /// This returns the stored value. To fetch current data, call `update()` first.
    pub fn is_charging(&self) -> bool {
        self.charging
    }

    /// Return the current battery voltage in 0.1 volts.
    ///
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
        let millivolts = self.read_millivolts().unwrap();
        if millivolts != self.millivolts {
            self.millivolts = millivolts;
            changed = true;
        }

        changed
    }


    // /// Convert a raw ADC measurement into a battery voltage in 0.1 volts.
    fn read_millivolts(&mut self) -> Option<i16> {
        let adc_val = self.saadc.read(&mut self.pin_voltage).unwrap();
        if adc_val < 0 {
            // What?
            return None;
        }
        Some((adc_val * 2000) / 4965) // we multiply the ADC value by 2 * 1000 for mV and divide by (2 ^ 14 / 3.3V reference)
    }

}