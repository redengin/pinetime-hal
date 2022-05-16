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
        );

        // clear the screen
        pinetime.lcd.clear(Rgb565::BLACK).unwrap();


        display_task::spawn().unwrap();

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

            // let text_style = MonoTextStyle::new(&FONT_6X9, Rgb565::GREEN);
            // Text::new("Hello World!", Point::new(0,0), text_style)
            //     .draw(&mut pinetime.lcd)
            //     .unwrap();
        });

        // run at 30Hz
        display_task::spawn_after(33.millis()).unwrap();
    }
}