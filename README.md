# icarus-firmware

This repo contains firmware packages for the [Icarus Flight Controller](https://github.com/nnarain/icarus)

Currently ESP MCU Rust support has two flavours: `std` and `bare-metal`.

The `std` HAL crate links to the original Espressif C SDK, while the `bare-metal` HAL crate is Rust from the ground up. Currently `std` is more feature complete, but I'd eventually like to move the `bare-metal` HAL. This repo will contain a mixture of both `std` and `bare-metal` firmware, but the main firmware (for now) will be `std`.

To use `std` firmware a fork of the Rust compiler is necessary (even though the ESP32-C3's RISC-V architecture is support by LLVM). A `vscode devcontainer` is provided to bring up a working development environment.

**icarus**

This package is the main board support package (bsp) for `bare-metal` firmware. It defines the available hardware on the board and does initialization of the USART, I2C, etc.

**icarus-app**

`bare-metal` firmware package.

**icarus-app-std**

`std` firmware package.

**icarus-wire**

Communication interface for controller

**icarus-cli**

Command line tool for interacting with controller.

**icarus-test**

Test bed package. Mostly contains simple examples and experiments.
