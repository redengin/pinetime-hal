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

### For BlueTooth (BLE) and Flash support choose an Operating System(OS)
As there are already robust implementations of these functionalities from an OS,
there is no reason to re-implement these functionalities more for bare-metal.

OS choices https://wiki.pine64.org/wiki/PineTime_Development  
**TockOs is implemented in Rust** https://www.tockos.org/documentation/walkthrough/



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


Setting up JTAG
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
Note, you may have to update your STLink firmware:
https://www.st.com/resource/en/data_brief/stsw-link007.pdf

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


**Alternatively**, you can use OpenOCD
--------------------------------------------------------------------------------
If your distro's version of OpenOCD isn't working, build it yourself.
```sh
git clone https://github.com/ntfreak/openocd --depth=1
cd openocd
# add hidapi for CMSSIS-DAP
git clone https://github.com/Dashlane/hidapi.git --depth=1
cd hidapi
./bootstrap && ./configure
make -j
# return to OpenOCD directory
cd ..
./bootstrap
# NOTE: where you see `linux` you can change to `mac`
HIDAPI_LIBS="-Lhidapi/linux/.libs -lhidapi-hidraw -lpthread" \
PKG_CONFIG_PATH=hidapi/pc/ \
CPPFLAGS="-Ihidapi/hidapi" \
    ./configure --enable-cmsis-dap
make -j
```
To start your own OpenOCD
```sh
# NOTE: use this from the project directory which provides an openocd.cfg
./openocd/src/openocd --search openocd/tcl/
```

### Testing out your OpenOcd
```sh
./openocd/src/openocd --search openocd/tcl/
# should result in
# ...
# Info : [nrf52.cpu] Cortex-M4 r0p1 processor detected
# Info : [nrf52.cpu] target has 6 breakpoints, 4 watchpoints
# ...
```
in another terminal (leaving the openocd process running)
```sh
gdb-multiarch -q target/thumbv7em-none-eabihf/debug/examples/rtic_demo
# within gdb
    target extended-remote :3333
    load
    continue
# you shouldn't see any errors in your `openocd` terminal
# the demo should run on the pinetime
```