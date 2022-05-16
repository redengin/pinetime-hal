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

Demo on the Pinetime
================================================================================
```sh
cargo build --example rtic_demo

# Using Rust's cargo-embed (see below for setup)
# NOTE: you'll need to run cargo embed from this project directory (which provides an Embed.toml)
cargo embed --example rtic_demo

# Using OpenOcd
# NOTE: you'll need to start openocd in another terminal (from this project directory)
openocd
cargo run --example rtic_demo
```

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

### To test it out
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
```