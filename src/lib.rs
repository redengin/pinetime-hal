#![no_std]
// embedded-hal traits
mod delay;
use delay::Delay;

// monotonic timer for rtic scheduling
pub mod monotonic_nrf52;

use nrf52832_hal::{self as hal, pac};
use hal::Spim;
use hal::gpio::{PushPull, PullUp};
use hal::saadc::{Saadc, SaadcConfig};
use hal::pac::SAADC;
use hal::gpio::{p0, Pin, Input, Output, Floating, Level};
use hal::pac::{SPIM0, TWIM1};
use hal::twim::{Twim};
use display_interface_spi::SPIInterface;
use st7789::{self, ST7789}; // LCD driver
use cst816s::CST816S;       // touchpad driver


mod battery_status;
use battery_status::BatteryStatus;
mod backlight;
use backlight::Backlight;
mod accelerometer;
use rubble_nrf5x::radio::{BleRadio, PacketBuffer};

pub const SCREEN_WIDTH: u32 = 240;
pub const SCREEN_HEIGHT: u32 = 240;

pub struct Pinetime {
    pub battery: BatteryStatus,
    pub backlight: Backlight,
    pub crown: Pin<Input<Floating>>,
    pub lcd: st7789::ST7789<
        SPIInterface<
            Spim<SPIM0>,
            p0::P0_18<Output<PushPull>>,    // data/command pin
            p0::P0_25<Output<PushPull>>,    // chip select
        >,
        p0::P0_26<Output<PushPull>>,        // reset pin
    >,
    pub touchpad: CST816S<Twim<TWIM1>,
        p0::P0_28<Input<PullUp>>,           // interrupt pin
        p0::P0_10<Output<PushPull>>,        // reset pin
    >,
    pub ble_radio: BleRadio,
    ble_tx_buffer: PacketBuffer,
    ble_rx_buffer: PacketBuffer,
}

impl Pinetime {
    pub fn init(
                hw_timer0: pac::TIMER0,
                hw_gpio: pac::P0,
                hw_saddc: SAADC,
                hw_spi0: pac::SPIM0,
                hw_twim1: pac::TWIM1,
                hw_ble_radio: pac::RADIO,
                hw_ficr: pac::FICR,
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
        // Set up SPI0
        let spi0_pins = hal::spim::Pins {
            sck: gpio.p0_02.into_push_pull_output(Level::Low).degrade(),
            mosi: Some(gpio.p0_03.into_push_pull_output(Level::Low).degrade()),
            miso: Some(gpio.p0_04.into_floating_input().degrade()),
        };
        let spi0 = hal::Spim::new(
            hw_spi0,
            spi0_pins,
            // 8MHz to maximize screen refresh
            hal::spim::Frequency::M8,
            hal::spim::MODE_3,
            0,  // fill transmissions with tailing zeros (TODO: should this be 122?)
        );
        // Set up LCD
        let lcd_cs = gpio.p0_25.into_push_pull_output(Level::High);
        let lcd_dc = gpio.p0_18.into_push_pull_output(Level::Low); // data/clock switch
        let lcd_di = SPIInterface::new(spi0, lcd_dc, lcd_cs);
        let lcd_rst = gpio.p0_26.into_push_pull_output(Level::Low); // reset pin
        let mut lcd= ST7789::new(lcd_di, lcd_rst, SCREEN_WIDTH as u16, SCREEN_HEIGHT as u16);
        let mut lcd_delay = Delay::init(hw_timer0);
        lcd.init(&mut lcd_delay).unwrap();
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

        // Set up Bluetooth TODO update rubble

        Self {
            battery,
            backlight: Backlight::init(
                gpio.p0_14.into_push_pull_output(Level::Low).degrade(),
                gpio.p0_22.into_push_pull_output(Level::Low).degrade(),
                gpio.p0_23.into_push_pull_output(Level::Low).degrade(),
            ),
            crown,
            lcd,
            touchpad,
            ble_radio,
            ble_tx_buffer,
            ble_rx_buffer,
        }
    }
}


