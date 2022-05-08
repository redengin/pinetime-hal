#![no_std]
// monotonic timer for rtic scheduling
pub mod monotonic_nrf52;

use hal::Spim;
use hal::gpio::{PushPull, PullUp};
use nrf52832_hal::{self as hal, pac};
use nrf52832_hal::saadc::{Saadc, SaadcConfig};
use nrf52832_hal::pac::SAADC;
use nrf52832_hal::gpio::{p0, Pin, Input, Output, Floating, Level};
use nrf52832_hal::pac::{SPIM1, TWIM1};
use nrf52832_hal::twim::{Twim};
use display_interface_spi::SPIInterface;
use st7789::{self, ST7789, Orientation};    // LCD driver
use cst816s::CST816S;                       // touchpad driver
// embedded-hal traits
mod delay;
use delay::Delay;

mod battery_status;
use battery_status::BatteryStatus;
mod backlight;
use backlight::Backlight;
mod accelerometer;


pub struct Pinetime {
    pub battery: BatteryStatus,
    pub backlight: Backlight,
    pub crown: Pin<Input<Floating>>,
    pub screen: st7789::ST7789<
        SPIInterface<
            Spim<SPIM1>,
            p0::P0_18<Output<PushPull>>,    // data/command pin
            p0::P0_25<Output<PushPull>>,    // chip select
        >,
        p0::P0_26<Output<PushPull>>,        // reset pin
    >,
    pub touchpad: CST816S<Twim<TWIM1>,
        p0::P0_28<Input<PullUp>>,           // interrupt pin
        p0::P0_10<Output<PushPull>>,        // reset pin
    >,
}

impl Pinetime {
    pub fn init(
                hw_timer0: pac::TIMER0,
                hw_gpio: pac::P0,
                hw_saddc: SAADC,
                hw_spi: pac::SPIM1,
                hw_twim1: pac::TWIM1,
    ) -> Self {
        // Set up GPIO
        let gpio = hal::gpio::p0::Parts::new(hw_gpio);
        // Set up battery status
        let saadc = Saadc::new(hw_saddc, SaadcConfig::default());
        let battery = BatteryStatus::init(
                saadc,
                gpio.p0_12.into_floating_input().degrade(),
                gpio.p0_31.into_floating_input(),
        );
        // enable crown
        gpio.p0_15.into_push_pull_output(Level::High);
        let crown = gpio.p0_13.into_floating_input().degrade();
        // Set up SPI
        let spi_pins = hal::spim::Pins {
            sck: gpio.p0_02.into_push_pull_output(Level::Low).degrade(),
            mosi: Some(gpio.p0_03.into_push_pull_output(Level::Low).degrade()),
            miso: Some(gpio.p0_04.into_floating_input().degrade()),
        };
        let spi1 = hal::Spim::new(
            hw_spi,
            spi_pins,
            // 8MHz to maximize screen refresh
            hal::spim::Frequency::M8,
            hal::spim::MODE_3,
            0,  // fill transmissions with tailing zeros (TODO: should this be 122?)
        );
        // Set up Screen
        let screen_cs = gpio.p0_25.into_push_pull_output(Level::High);
        let dc = gpio.p0_18.into_push_pull_output(Level::Low); // data/clock switch
        let di = SPIInterface::new(spi1, dc, screen_cs);
        let rst = gpio.p0_26.into_push_pull_output(Level::Low); // reset pin
        let mut screen = ST7789::new(di, rst, 240, 240);
        let mut screen_delay = Delay::init(hw_timer0);
        screen.init(&mut screen_delay).unwrap();
        screen.set_orientation(Orientation::Landscape).unwrap();
        // Set up Touchpad
        let i2c_pins = hal::twim::Pins {
            scl: gpio.p0_07.into_floating_input().degrade(),
            sda: gpio.p0_06.into_floating_input().degrade(),
        };
        let i2c_port = hal::twim::Twim::new(hw_twim1, i2c_pins, hal::twim::Frequency::K400);
        let touch_interrupt_pin = gpio.p0_28.into_pullup_input();
        let touch_rst = gpio.p0_10.into_push_pull_output(Level::High);
        let touchpad = CST816S::new(i2c_port, touch_interrupt_pin, touch_rst);
        // Set up accelerometer TODO: implement

        Self {
            battery,
            backlight: Backlight::init(
                gpio.p0_14.into_push_pull_output(Level::Low).degrade(),
                gpio.p0_22.into_push_pull_output(Level::Low).degrade(),
                gpio.p0_23.into_push_pull_output(Level::Low).degrade(),
            ),
            crown,
            screen,
            touchpad,
        }
    }
}


