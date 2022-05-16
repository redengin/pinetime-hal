#![no_std]
#![no_main]

use panic_rtt_target as _;


#[rtic::app(device = nrf52832_hal::pac, peripherals = true, dispatchers = [SWI0_EGU0])]
mod app {
    use nrf52832_hal as _;
    use rtt_target::{rprintln, rtt_init_print};
    use pinetime_hal::monotonic_nrf52::{MonoTimer};
    use fugit::{self, ExtU32};
    use pinetime_hal::Pinetime;

    use embedded_graphics::{
        prelude::*,
        pixelcolor::Rgb565,
        mono_font::{MonoTextStyle, ascii::FONT_6X10},
        text::Text,
    };

    #[monotonic(binds = TIMER1, default = true)]
    type Tonic = MonoTimer<nrf52832_hal::pac::TIMER1>;

    #[shared]
    struct Shared {
    }


    #[local]
    struct Local {
        pinetime: pinetime_hal::Pinetime,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        // initialize scheduler timer
        let mono = MonoTimer::new(cx.device.TIMER1);

        let pinetime = Pinetime::init(
            cx.device.TIMER0,
            cx.device.P0,
            cx.device.SAADC,
            cx.device.SPIM0,
            cx.device.TWIM1,
        );

        display_task::spawn().unwrap();

        ( Shared {
          },
          Local {
            pinetime,
          },
          init::Monotonics(mono)
        )
    }

    #[task(local=[pinetime])]
    fn display_task(cx: display_task::Context) {
        let pinetime = cx.local.pinetime;

        // set the backlight
        pinetime.backlight.set(3);

        // clear the screen
        pinetime.lcd.clear(Rgb565::BLACK).unwrap();

        let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
        Text::new("Pinetime", Point::new(0, 6), text_style)
            .draw(&mut pinetime.lcd)
            .unwrap();
    }
}