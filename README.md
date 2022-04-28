Pinetime Board Support Packet (in a crate)
================================================================================

Hardware
--------------------------------------------------------------------------------
* nRF52832 SOC (`thumbv7em-none-eabihf`)


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