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
        text::Text,
        mono_font::{ascii::FONT_6X9, MonoTextStyle},
    };

    #[monotonic(binds = TIMER1, default = true)]
    type Tonic = MonoTimer<nrf52832_hal::pac::TIMER1>;

    #[shared]
    struct Shared {
        // TODO decouple pinetime properties per `local` task need
        pinetime: pinetime_hal::Pinetime,
    }


    #[local]
    struct Local {
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        // initialize scheduler timer
        let mono = MonoTimer::new(cx.device.TIMER1);

        let mut pinetime = Pinetime::init(
            cx.device.TIMER0,
            cx.device.P0,
            cx.device.SAADC,
            cx.device.SPIM1,
            cx.device.TWIM1,
        );
        pinetime.backlight.set(1);

        rtt_init_print!();
        rprintln!("init");

        display_task::spawn().ok();

        ( Shared {
            pinetime,
          },
          Local {
          },
          init::Monotonics(mono)
        )
    }

    #[task(shared=[pinetime])]
    fn display_task(cx: display_task::Context) {
        let mut pinetime = cx.shared.pinetime;
        (pinetime).lock(|pinetime| {
            pinetime.backlight.set(7);

            let text_style = MonoTextStyle::new(&FONT_6X9, Rgb565::GREEN);
            Text::new("Hello World!", Point::new(0,0), text_style)
                .draw(&mut pinetime.screen)
                .unwrap();
        });

        // run at 30Hz
        display_task::spawn_after(33.millis()).unwrap();
    }
}