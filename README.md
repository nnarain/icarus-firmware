# icarus-firmware

This repo contains firmware packages for the [Icarus controller](https://github.com/nnarain/icarus)

**icarus**

This package is the main board support package (bsp). It defines the available hardware on the board and does initialization of the USART, I2C, etc.

**icarus-app**

This is the application firmware package.

**icarus-test**

Test bed package. Mostly contains simple examples and experiments.

**Example**

Install `cargo-just`

Run examples:

```
just run-example <example name>
```
