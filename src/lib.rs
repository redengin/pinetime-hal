use nrf52832_hal::{self as hal, pac};
use nrf52832_hal::saadc::{Saadc, SaadcConfig};
use nrf52832_hal::target::SAADC;
use nrf52832_hal::gpio::Level;
// embedded-hal traits
mod delay;
use delay::Delay;


pub mod battery_status;
use battery_status::BatteryStatus;
mod backlight;
use backlight::Backlight;


pub struct Pinetime {
    pub delay: Delay,
    pub battery: BatteryStatus,
    pub backlight: Backlight,
}

impl Pinetime {
    pub fn init(
                hw_timer0: pac::TIMER0,
                hw_gpio: pac::P0,
                hw_saddc: SAADC,
    ) -> Self {
        // Set up GPIO
        let gpio = hal::gpio::p0::Parts::new(hw_gpio);
        // Set up ADC
        let saadc = Saadc::new(hw_saddc, SaadcConfig::default());

        Self {
            delay: delay::Delay::init(hw_timer0),
            battery: BatteryStatus::init(
                saadc,
                gpio.p0_12.into_floating_input().degrade(),
                gpio.p0_31.into_floating_input(),
            ),
            backlight: Backlight::init(
                gpio.p0_14.into_push_pull_output(Level::Low).degrade(),
                gpio.p0_22.into_push_pull_output(Level::Low).degrade(),
                gpio.p0_23.into_push_pull_output(Level::Low).degrade(),
            ),
        }
    }
}
