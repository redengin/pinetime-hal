use embedded_hal::adc::OneShot;
use embedded_hal::digital::v2::InputPin;
use nrf52832_hal::gpio::{p0, Pin, Input, Floating};
use nrf52832_hal::saadc::{Saadc};
use fixed::types::U4F12;

pub struct Battery {
    /// Pin High = battery, Low = charging.
    pin_charge_indication: Pin<Input<Floating>>,

    /// SAADC peripheral
    saadc: Saadc,

    /// Pin Voltage level
    pin_voltage: p0::P0_31<Input<Floating>>,
}

impl Battery {
    pub fn new(pin_charge_indication: Pin<Input<Floating>>,
               saadc: Saadc,
               pin_voltage: p0::P0_31<Input<Floating>>) -> Self {
        Self {
            pin_charge_indication,
            saadc,
            pin_voltage
        }
    }

    pub fn is_charging(&self) -> Result<bool, ()> {
        return match self.pin_charge_indication.is_low() {
            Ok(val) => Ok(val),
            Err(_) => Err(())
        }
    }

    pub fn voltage(&mut self) -> Result<U4F12, ()> {
        return match self.saadc.read(&mut self.pin_voltage) {
            Ok(val) => Ok((U4F12::from_bits(val as u16 + 1) / 2 / 10) * 33),
            Err(_) => Err(())
        };
    }
}