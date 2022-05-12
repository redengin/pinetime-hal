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
    pub charging: bool,

    pub voltage: f32,
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
            voltage: 0 as f32,
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
    pub fn voltage(&self) -> f32 {
        self.voltage
    }

    /// Update the current battery status by reading information from the hardware.
    pub fn update(&mut self) {
        // Check charging status
        self.charging =  self.pin_charge_indication.is_low().unwrap();

        // Check voltage
        self.voltage = self.read_voltage();
    }

    /// Convert a raw ADC measurement into a battery voltage
    fn read_voltage(&mut self) -> f32 {
        let adc_read = self.saadc.read(&mut self.pin_voltage);
        match adc_read {
            Ok(val) => return (val as f32 * 2.0) / (4095.0 / 3.3),
            Err(_) => return 0 as f32
        };
    }
}