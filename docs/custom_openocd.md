Create a PineTime compatible OpenOCD binary
================================================================================
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

