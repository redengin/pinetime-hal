#![no_std]
#![no_main]

use panic_rtt_target as _;


#[rtic::app(device = nrf52832_hal::pac, peripherals = true, dispatchers = [SWI0_EGU0])]
mod app {
    use rtt_target::{rprintln, rtt_init_print};
    use pinetime_hal::monotonic_nrf52::{MonoTimer};
    // use fugit::{self, ExtU32};
    use pinetime_hal::Pinetime;

    use embedded_graphics::{
        prelude::*,
        pixelcolor::Rgb565,
        mono_font::{MonoTextStyle, ascii::FONT_10X20},
        text::Text,
    };

    #[monotonic(binds = TIMER1, default = true)]
    type Tonic = MonoTimer<nrf52832_hal::pac::TIMER1>;

    #[shared]
    struct Shared {
        // under RTIC, shared busses need to be locked https://github.com/ryan-summers/shared-bus-rtic
        spi_peripherals: pinetime_hal::SharedSpi,
        i2c_peripherals: pinetime_hal::SharedI2c,
    }


    #[local]
    struct Local {
        battery: pinetime_hal::battery_status::BatteryStatus,
        backlight: pinetime_hal::backlight::Backlight,
        // crown: Pin<Input<Floating>>,
        vibrator: pinetime_hal::vibrator::Vibrator,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        // initialize scheduler timer
        let mono = MonoTimer::new(cx.device.TIMER1);

        let mut pinetime = Pinetime::init(
            cx.device.TIMER0,
            cx.device.P0,
            cx.device.SAADC,
            cx.device.SPIM0,
            cx.device.TWIM1,
            cx.device.RADIO,
            cx.device.FICR,
        );

        // clear the display
        pinetime.lcd.clear(Rgb565::BLACK).unwrap();
        // set the backlight
        pinetime.backlight.set(3);

        // spawn initial tasks
        display_task::spawn().unwrap();

        ( Shared {
            spi_peripherals: pinetime_hal::SharedSpi {
                lcd: pinetime.lcd,
            },
            i2c_peripherals: pinetime_hal::SharedI2c {
                touchpad: pinetime.touchpad,
                heartrate: pinetime.heartrate,
            }
          },
          Local {
            battery: pinetime.battery,
            backlight: pinetime.backlight,
            vibrator: pinetime.vibrator,
          },
          init::Monotonics(mono)
        )
    }

    #[task(shared=[spi_peripherals], local=[backlight])]
    fn display_task(mut cx: display_task::Context) {

        let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
        cx.shared.spi_peripherals.lock(|bus| {

            Text::new("Pinetime", Point::new(0, 15), text_style)
                .draw(&mut bus.lcd)
                .unwrap();
        });
    }
}