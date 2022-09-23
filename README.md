Rust Pinetime Hardware Abstraction Layer (HAL)
================================================================================

Hardware Support
--------------------------------------------------------------------------------
* [nRF52832 SOC](https://crates.io/crates/nrf52832-hal)
* [Battery](src/battery.rs)
* [Backlight](src/backlight.rs)
* [LCD](https://crates.io/crates/st7789)
* [Touch Screen](https://crates.io/crates/cst816s)
* [Heart Rate Sensor](https://crates.io/crates/hrs3300)
* [Vibration](src/vibrator.rs)
* [Temperature](src/temperature.rs)
* Accelerometer (TODO)
* Bluetooth (BLE)
    * Use [embedded OS that supports PineTime](https://wiki.pine64.org/wiki/PineTime_Development).
        * NOTE: you will need to understand how to integrate these Rust implementations into the
            chosen environment.
    * **work** on the development of a [Rust BLE implementation](https://github.com/redengin/embedded-ble)
* Filesystem - use of remaining flash (not used for static application) for storage 
    * Use [embedded OS that supports PineTime](https://wiki.pine64.org/wiki/PineTime_Development).
        * NOTE: you will need to understand how to integrate these Rust implementations into the
            chosen environment.


Demo on the Pinetime
================================================================================
Using Probe-rs (recommended)
--------------------------------------------------------------------------------
```sh
# Using Rust's cargo-embed (see below for setup)
# NOTE: you'll need to run cargo embed from this project directory (which provides an Embed.toml)
cargo embed --release --example rtic_demo
```
Using OpenOcd
--------------------------------------------------------------------------------
```sh
# NOTE: you'll need to start openocd in another terminal
cargo run --release --example rtic_demo
```
**Tapping on the screen** marks the touch with an **X**.


Setting up access to Pinetime
================================================================================
The **preferred method** is to use [probe-rs](https://probe.rs/docs/getting-started/probe-setup/).
--------------------------------------------------------------------------------
`Probe.rs` is a rust implementation that provides both a
debugger (i.e. what gdb does) as well as support for common
debugger hardware (i.e. what openocd does).
```sh
# install the flashing utilities
cargo install cargo-flash
# install the advanced embedded debug utilities
cargo install cargo-embed
```
### Note, you may have to update your STLink firmware:
https://www.st.com/resource/en/data_brief/stsw-link007.pdf

<!--
### [Integrating Probe-rs with VSCode](https://probe.rs/docs/tools/vscode/)
```sh
# update url to latest available
curl -L https://github.com/probe-rs/vscode/releases/download/v0.4.0/probe-rs-debugger-0.4.0.vsix \
    --output probe-rs-debugger.vsix
# install it into VSCode
code --install-extension probe-rs-debugger.vsix
# install the probe-rs debugger
cargo install --git https://github.com/probe-rs/probe-rs probe-rs-debugger
```
**TODO** the `.vscode/launch.json` is not currently working.
-->

Using OpenOcd
--------------------------------------------------------------------------------
If your OS provided version of OpenOcd doesn't work, use [custom_ocd](docs/custom_openocd.md).
