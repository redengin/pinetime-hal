use embedded_hal::adc::OneShot;
use embedded_hal::digital::v2::InputPin;
use nrf52832_hal::gpio::{p0, Pin, Input, Floating};
use nrf52832_hal::saadc::{Saadc};

pub struct Battery {
    /// Pin High = battery, Low = charging.
    pub(super) pin_charge_indication: Pin<Input<Floating>>,

    /// SAADC peripheral
    pub(super) saadc: Saadc,

    /// Pin Voltage level
    pub(super) pin_voltage: p0::P0_31<Input<Floating>>,
}

impl Battery {
    pub fn is_charging(&self) -> Result<bool, ()> {
        return match self.pin_charge_indication.is_low() {
            Ok(val) => Ok(val),
            Err(_) => Err(())
        }
    }

    pub fn millivolts(&mut self) -> Result<i32, ()> {
        const FUDGE:i32 = 2;    // not sure why but this makes the reading correct
        const ADC_SCALE:i32 = FUDGE * (4095.0 / 3.3) as i32;
        const MV_PER_VOLT:i32 = 1000;
        return match self.saadc.read(&mut self.pin_voltage) {
            Ok(val) => Ok((val as i32 * MV_PER_VOLT) / ADC_SCALE),
            Err(_) => Err(())
        };
    }
}