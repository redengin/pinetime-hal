Rust Pinetime Hardware Abstraction Layer (HAL)
================================================================================

Hardware
--------------------------------------------------------------------------------
* nRF52832 SOC (`thumbv7em-none-eabihf`)
* embedded-hal traits
    * [Delay](src/delay.rs)
* [Battery](src/battery.rs)
* [Backlight](src/backlight.rs)
* [Screen](https://crates.io/crates/st7789)
    * [embedded-graphics](https://crates.io/crates/embedded-graphics)
* [Touch Screen](https://crates.io/crates/cst816s)
* [Accelerometer](src/accelerometer.rs)
* Heart Rate Sensor (TODO)
* Vibration (TODO)
* Flash (TODO) (PM25LV512) (https://crates.io/crates/spi-memory)

Demo
================================================================================
```sh
cargo build --example rtic_demo --target thumbv7em-none-eabihf
cargo run --example rtic_demo --target thumbv7em-none-eabihf
```



On-Target Test
================================================================================
```sh
cargo build --example test_on_host --target thumbv7em-none-eabihf
cargo run --example test_on_host --target thumbv7em-none-eabihf
```