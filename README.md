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
# NOTE: you'll need to start openocd in another terminal (from this project directory)
openocd
cargo run --example rtic_demo --target thumbv7em-none-eabihf
```



On-Target Test
================================================================================
```sh
cargo build --example test_on_host --target thumbv7em-none-eabihf
# NOTE: you'll need to start openocd in another terminal (from this project directory)
openocd
cargo run --example test_on_host --target thumbv7em-none-eabihf
```


Setting up OpenOCD for Pinetime
--------------------------------------------------------------------------------
If your distro's version of OpenOCD isn't working, use the following to build
a newer version.
```sh
# install hidapi (necessary for CMSIS-DAP)
git clone https://github.com/Dashlane/hidapi.git --depth=1
cd hidapi
./bootstrap && ./configure
make -j
sudo make install

# install openocd
git clone https://github.com/ntfreak/openocd --depth=1
cd openocd
./bootstrap && HIDAPI_LIBS="-lhidapi-hidraw -lpthread" \
    ./configure --enable-cmsis-dap
make -j
sudo make install
```

### To test it out
```sh
openocd
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