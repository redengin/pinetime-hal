use hal::Spim;
use nrf52832_hal::{self as hal, pac};
use nrf52832_hal::saadc::{Saadc, SaadcConfig};
use nrf52832_hal::target::SAADC;
use nrf52832_hal::gpio::{Pin, Input, Floating, Level};
use nrf52832_hal::target::SPIM1;
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
    pub crown: Pin<Input<Floating>>,
    spi1: Spim<SPIM1>,
}

impl Pinetime {
    pub fn init(
                hw_timer0: pac::TIMER0,
                hw_gpio: pac::P0,
                hw_saddc: SAADC,
                hw_spi: pac::SPIM1,
    ) -> Self {
        // Set up GPIO
        let gpio = hal::gpio::p0::Parts::new(hw_gpio);
        // Set up ADC
        let saadc = Saadc::new(hw_saddc, SaadcConfig::default());
        // enable crown
        gpio.p0_15.into_push_pull_output(Level::High);
        // Set up SPI
        let spi_clk = gpio.p0_02.into_push_pull_output(Level::Low).degrade();
        let spi_mosi = gpio.p0_03.into_push_pull_output(Level::Low).degrade();
        let spi_miso = gpio.p0_04.into_floating_input().degrade();
        let spi_pins = hal::spim::Pins {
            sck: spi_clk,
            miso: Some(spi_miso),
            mosi: Some(spi_mosi),
        };
        let spi1 = hal::Spim::new(
            hw_spi,
            spi_pins,
            // 8MHz to maximize screen refresh
            hal::spim::Frequency::M8,
            hal::spim::MODE_3,
            0,
        );
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
            crown: gpio.p0_13.into_floating_input().degrade(),
            spi1: spi1,
        }
    }
}
