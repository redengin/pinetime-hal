#![no_std]
#![no_main]

use panic_rtt_target as _;


#[rtic::app(device = nrf52832_hal::pac, peripherals = true, dispatchers = [SWI0_EGU0])]
mod app {
    use pinetime_hal::Pinetime;
    use pinetime_hal::monotonic_nrf52::{MonoTimer};
    // use fugit::{self, ExtU32};
    use embedded_graphics::{
        prelude::*,
        pixelcolor::Rgb565,
        mono_font::{MonoTextStyleBuilder, ascii::FONT_10X20},
        text::{Text},
    };
    use heapless::String;
    use core::fmt::Write;
    use rtt_target::{rprintln, rtt_init_print};

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
        battery: pinetime_hal::battery::Battery,
        // backlight: pinetime_hal::backlight::Backlight,
        // crown: Pin<Input<Floating>>,
        // vibrator: pinetime_hal::vibrator::Vibrator,
        temperature: nrf52832_hal::Temp,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        // initialize scheduler timer
        let mono = MonoTimer::new(cx.device.TIMER1);

        let mut pinetime = Pinetime::init(
            cx.device.P0,
            cx.device.SAADC,
            cx.device.TEMP,
            cx.device.SPIM0,
            cx.device.TIMER0,
            cx.device.TWIM1,
            cx.device.CLOCK,
            cx.device.RADIO,
            cx.device.FICR,
        );
        
        // Setup Bluetooth TODO: implement

        // clear the display
        pinetime.lcd.clear(Rgb565::BLACK).unwrap();
        // set the backlight
        pinetime.backlight.set(1);

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
            // backlight: pinetime.backlight,
            // vibrator: pinetime.vibrator,
            temperature: pinetime.temperature,
          },
          init::Monotonics(mono)
        )
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            // go into deep sleep to conserve power
            cortex_m::asm::wfe();
        }
    }

    #[task(shared=[spi_peripherals, i2c_peripherals], local=[battery, temperature])]
    fn display_task(mut cx: display_task::Context) {

        let voltage = cx.local.battery.voltage();
        let charging = cx.local.battery.is_charging();
        let temperature = cx.local.temperature.measure();
        let mut touchEvent = None;
        cx.shared.i2c_peripherals.lock(|i2c| {
            touchEvent = i2c.touchpad.read_one_touch_event(false);
        });

        // create a non-transparent font
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(Rgb565::WHITE)
            .background_color(Rgb565::BLACK)
            .build();

        cx.shared.spi_peripherals.lock(|spi| {
            // display a header
            Text::new("Pinetime", Point::new(0, 15), text_style)
                .draw(&mut spi.lcd)
                .unwrap();

            // display charging status
            let mut charging_text = String::<50>::new();
            match charging {
                Ok(value) => { write!(charging_text, "charging {:3}", value).unwrap() },
                Err(_) => { write!(charging_text, "failed").unwrap() },
            };
            Text::new(charging_text.as_str(), Point::new(25, 40), text_style)
                .draw(&mut spi.lcd)
                .unwrap();

            // display voltage status
            let mut voltage_text = String::<50>::new();
            match voltage {
                Ok(value) => {
                     write!(voltage_text, "battery {:03.3} V", value).unwrap()
                }, 
                Err(_) => { write!(voltage_text, "battery failed").unwrap() },
            };
            Text::new(voltage_text.as_str(), Point::new(25, 60), text_style)
                .draw(&mut spi.lcd)
                .unwrap();

            // display temperature
            let mut temperature_text = String::<50>::new();
            write!(temperature_text, "temperature {:03.2} C", temperature).unwrap();
            Text::new(temperature_text.as_str(), Point::new(25, 80), text_style)
                .draw(&mut spi.lcd)
                .unwrap();

            // display touch point
            match touchEvent {
                Some(touchEvent) => { Text::new("X", Point::new(touchEvent.x, touchEvent.y), text_style)
                                                    .draw(&mut spi.lcd).unwrap();
                                    },
                None => {},
            };
        });

        display_task::spawn().unwrap();
    }
}