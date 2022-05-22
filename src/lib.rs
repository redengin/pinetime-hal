#![no_std]
// monotonic timer for rtic scheduling
pub mod monotonic_nrf52;

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
use rubble::{
    config::Config,
    security::NoSecurity,
    l2cap::{BleChannelMap},
    link::{
        ad_structure::AdStructure,
        queue::{PacketQueue, SimpleQueue},
        LinkLayer, MIN_PDU_BUF,
    },
    time::{Duration},
};
use rubble_nrf5x::{
    radio::{BleRadio, PacketBuffer},
    timer::BleTimer,
};
pub mod ble;

pub mod vibrator;
use vibrator::Vibrator;
pub mod battery_status;
use battery_status::BatteryStatus;
pub mod backlight;
use backlight::Backlight;
pub mod accelerometer;

pub const SCREEN_WIDTH: u32 = 240;
pub const SCREEN_HEIGHT: u32 = 240;

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

pub enum BleConfig {}

impl Config for BleConfig {
    type Timer = BleTimer<hal::pac::TIMER0>;
    type Transmitter = BleRadio;
    type ChannelMapper = BleChannelMap<ble::BleAttributes, NoSecurity>;
    type PacketQueue = &'static mut SimpleQueue;
}

pub struct Pinetime {
    pub battery: BatteryStatus,
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
    pub ble_radio: Option<BleRadio>, 
    pub ble_linklayer: Option<LinkLayer<BleConfig>>,
}

impl Pinetime {
    pub fn init(
                hw_gpio: pac::P0,
                hw_saddc: SAADC,
                hw_temperature: pac::TEMP,
                hw_spi: pac::SPIM0,    // note: SPIM1 locks waiting for interrupt under display driver
                hw_timer0: pac::TIMER0,
                hw_i2c: pac::TWIM1,
                hw_clock: pac::CLOCK,
                hw_ble_radio: pac::RADIO,
                hw_ficr: pac::FICR,
    ) -> Self {
        // Set up GPIO
        let gpio = hal::gpio::p0::Parts::new(hw_gpio);

        // enable crown (button)
        gpio.p0_15.into_push_pull_output(Level::High);

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

        // Set up accelerometer TODO: implement

        // Set up heartrate sensor
        let mut heartrate = Hrs3300::new(i2c_bus.acquire());
        heartrate.init().unwrap();

        // Set up temperature sensor
        let temperature = hal::Temp::new(hw_temperature);

        // Set up Bluetooth
        // Switch to external HF oscillator for Bluetooth
        hal::clocks::Clocks::new(hw_clock).enable_ext_hfosc();
        let hw_timer0 = timer.free();   // take back the timer
        let ble_timer = BleTimer::init(hw_timer0);
        let ble_address = rubble_nrf5x::utils::get_device_address();
        static mut BLE_TX_BUF:PacketBuffer = [0; MIN_PDU_BUF];
        static mut BLE_RX_BUF:PacketBuffer = [0; MIN_PDU_BUF];
        static mut BLE_TX_QUEUE:SimpleQueue = SimpleQueue::new();
        static mut BLE_RX_QUEUE:SimpleQueue = SimpleQueue::new();
        let ble_radio: Option<BleRadio> = None;
        let ble_linklayer: Option<LinkLayer<BleConfig>> = None;
        unsafe { // allow static buffer references to be taken (due to poor rubble architecture)
            let mut ble_radio = BleRadio::new(
                hw_ble_radio,
                &hw_ficr,
                &mut BLE_TX_BUF,
                &mut BLE_RX_BUF,
            );
            let mut ble_linklayer = LinkLayer::<BleConfig>::new(ble_address, ble_timer);
            // Send advertisement and set up regular interrupt
            let (_, ble_tx_consumer) = BLE_TX_QUEUE.split();
            let (ble_rx_producer, _) = BLE_RX_QUEUE.split();
            ble_linklayer
                .start_advertise(
                    Duration::from_millis(200),
                    &[AdStructure::CompleteLocalName("Pinetime")],
                    &mut ble_radio,
                    ble_tx_consumer,
                    ble_rx_producer,
                )
                .unwrap();
        }

        Self {
            battery: BatteryStatus{
                saadc: Saadc::new(hw_saddc, SaadcConfig::default()),
                pin_charge_indication: gpio.p0_12.into_floating_input().degrade(),
                pin_voltage: gpio.p0_31.into_floating_input(),
            },
            crown: gpio.p0_13.into_floating_input().degrade(),
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
            ble_radio,
            ble_linklayer,
        }
    }
}