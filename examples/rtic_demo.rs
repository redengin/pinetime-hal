#![no_std]
#![no_main]

use nrf52832_hal as _;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use pinetime_hal::monotonic_nrf52::{MonoTimer};


#[rtic::app(device = nrf52832_hal::pac, peripherals = true, dispatchers = [SWI0_EGU0])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[monotonic(binds = TIMER1, default = true)]
    type Tonic = MonoTimer<nrf52832_hal::pac::TIMER1>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mono = MonoTimer::new(cx.device.TIMER1);

        rtt_init_print!();
        rprintln!("init");

        task1::spawn().ok();

        (Shared {}, Local {}, init::Monotonics(mono))
    }

    #[task]
    fn task1(_cx: task1::Context) {
        rprintln!("task1");
        // task1::spawn_after(2000.millis()).ok();
    }
}