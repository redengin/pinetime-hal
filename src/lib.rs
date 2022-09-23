#![no_std]

use nrf52832_hal::{self as hal, pac};
use hal::gpio::{p0, Pin, PushPull, PullUp, Input, Output, Floating, Level};
use hal::pac::{SAADC, SPIM0, TWIM1};
use hal::saadc::{Saadc, SaadcConfig};
use hal::Spim;
use hal::twim::Twim;
use shared_bus_rtic::SharedBus;
use st7789::ST7789;
use hrs3300::Hrs3300;
use display_interface_spi::SPIInterface;
use cst816s::CST816S;       // touchpad driver

pub mod vibrator;
use vibrator::Vibrator;
pub mod battery;
use battery::Battery;
pub mod backlight;
use backlight::Backlight;

pub const SCREEN_WIDTH: u32 = 240;
pub const SCREEN_HEIGHT: u32 = 240;

pub struct Pinetime {
    pub battery: Battery,
    pub crown: Pin<Input<Floating>>,
    pub vibrator: Vibrator,
    pub backlight: Backlight,
    pub lcd: st7789::ST7789<
        SPIInterface<
            SharedBus<Spim<SPIM0>>,
            p0::P0_18<Output<PushPull>>,    // data/command pin
            p0::P0_25<Output<PushPull>>,    // chip select
        >,
        p0::P0_26<Output<PushPull>>,        // reset pin
    >,
    pub touchpad: CST816S<SharedBus<Twim<TWIM1>>,
        p0::P0_28<Input<PullUp>>,           // interrupt pin
        p0::P0_10<Output<PushPull>>,        // reset pin
    >,
    pub heartrate: Hrs3300<SharedBus<Twim<TWIM1>>>,
    pub temperature: nrf52832_hal::Temp,
}

impl Pinetime {
    pub fn init(hw_gpio: pac::P0,
                hw_saddc: SAADC,
                hw_temperature: pac::TEMP,
                hw_spi: pac::SPIM0,    // note: SPIM1 locks waiting for interrupt under display driver
                hw_timer0: pac::TIMER0,
                hw_i2c: pac::TWIM1,
                _hw_clock: pac::CLOCK,
                _hw_ble_radio: pac::RADIO,
                _hw_ficr: pac::FICR,
    ) -> Self {
        // Set up GPIO
        let gpio = hal::gpio::p0::Parts::new(hw_gpio);

        // enable crown (button)
        gpio.p0_15.into_push_pull_output(Level::High);
        let crown = gpio.p0_13.into_floating_input().degrade();

        // enable battery monitor
        let saadc = Saadc::new(hw_saddc, SaadcConfig::default());
        let pin_charge_indication = gpio.p0_12.into_floating_input().degrade();
        let pin_voltage = gpio.p0_31.into_floating_input();
        let battery = Battery::new(pin_charge_indication, saadc, pin_voltage);

        // Set up SPI
        let spi_pins = hal::spim::Pins {
            sck: gpio.p0_02.into_push_pull_output(Level::Low).degrade(),
            mosi: Some(gpio.p0_03.into_push_pull_output(Level::Low).degrade()),
            miso: Some(gpio.p0_04.into_floating_input().degrade()),
        };
        let spim = hal::Spim::new(
            hw_spi,
            spi_pins,
            // 8MHz to maximize screen refresh
            hal::spim::Frequency::M8,
            hal::spim::MODE_3,
            0,
        );
        let spi_bus = shared_bus_rtic::new!(spim, Spim<SPIM0>);

        // Set up LCD
        let lcd_cs = gpio.p0_25.into_push_pull_output(Level::High);
        let lcd_dc = gpio.p0_18.into_push_pull_output(Level::Low); // data/clock switch
        let lcd_di = SPIInterface::new(spi_bus, lcd_dc, lcd_cs);
        let lcd_rst = gpio.p0_26.into_push_pull_output(Level::Low); // reset pin
        let mut lcd= ST7789::new(lcd_di, lcd_rst, SCREEN_WIDTH as u16, SCREEN_HEIGHT as u16);
        let mut timer= hal::timer::Timer::new(hw_timer0);
        lcd.init(&mut timer).unwrap();

        // Set up I2C
        let i2c_pins = hal::twim::Pins {
            scl: gpio.p0_07.into_floating_input().degrade(),
            sda: gpio.p0_06.into_floating_input().degrade(),
        };
        let i2c_twim= hal::twim::Twim::new(hw_i2c, i2c_pins, hal::twim::Frequency::K400);
        let i2c_bus = shared_bus_rtic::new!(i2c_twim, Twim<TWIM1>);

        // Set up Touchpad
        let touch_interrupt_pin = gpio.p0_28.into_pullup_input();
        let touch_rst = gpio.p0_10.into_push_pull_output(Level::High);
        let mut touchpad = CST816S::new(i2c_bus.acquire(), touch_interrupt_pin, touch_rst);
        touchpad.setup(&mut timer).unwrap();

        // Set up accelerometer
        // let accelerometer_interrupt_pin1 = Some(gpio.p0_08.into_pullup_input());
        // let accelerometer = accelerometer::BMA4xx::new(
        //     i2c_bus.acquire(),
        //     accelerometer_interrupt_pin1,
        //     None);

        // Set up heartrate sensor
        let mut heartrate = Hrs3300::new(i2c_bus.acquire());
        heartrate.init().unwrap();

        // Set up temperature sensor
        let temperature = hal::Temp::new(hw_temperature);

        // Set up Bluetooth TODO: implement

        Self {
            battery,
            crown,
            vibrator: Vibrator{
                pin: gpio.p0_16.into_push_pull_output(Level::High).degrade(),
            },
            backlight: Backlight{
                low: gpio.p0_14.into_push_pull_output(Level::Low).degrade(),
                mid: gpio.p0_22.into_push_pull_output(Level::Low).degrade(),
                high: gpio.p0_23.into_push_pull_output(Level::Low).degrade(),
            },
            lcd,
            touchpad,
            heartrate,
            temperature,
        }
    }
}

// under RTIC, shared busses need to be locked https://github.com/ryan-summers/shared-bus-rtic
pub struct SharedSpi {
    pub lcd: st7789::ST7789<
        SPIInterface<
            SharedBus<hal::Spim<pac::SPIM0>>,
            p0::P0_18<Output<PushPull>>,    // data/command pin
            p0::P0_25<Output<PushPull>>,    // chip select
        >,
        p0::P0_26<Output<PushPull>>,        // reset pin
    >,
    // TODO add flash
}

// under RTIC, shared busses need to be locked https://github.com/ryan-summers/shared-bus-rtic
pub struct SharedI2c {
    pub touchpad: CST816S<SharedBus<Twim<TWIM1>>,
        p0::P0_28<Input<PullUp>>,           // interrupt pin
        p0::P0_10<Output<PushPull>>,        // reset pin
    >,
    pub heartrate: Hrs3300<SharedBus<Twim<TWIM1>>>,
}
