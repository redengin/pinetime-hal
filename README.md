Rust Pinetime Hardware Abstraction Layer (HAL)
================================================================================

Hardware
--------------------------------------------------------------------------------
* nRF52832 SOC (`thumbv7em-none-eabihf`)
* embedded-hal traits
    * [Delay](src/delay.rs)
* [Battery](src/battery.rs)
* [Backlight](src/backlight.rs)
* [Screen](src/screen.rs)


Demo
================================================================================
```sh
cargo build --example demo --target thumbv7em-none-eabihf
cargo run --example demo --target thumbv7em-none-eabihf
```



On-Target Test
================================================================================
```sh
cargo build --example test_on_host --target thumbv7em-none-eabihf
cargo run --example test_on_host --target thumbv7em-none-eabihf
```