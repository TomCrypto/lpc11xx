# LPC11xx

[![Documentation](https://docs.rs/lpc11xx/badge.svg)](https://docs.rs/lpc11xx)
[![Crates.io](https://img.shields.io/crates/v/lpc11xx.svg)](https://crates.io/crates/lpc11xx)

Register mappings for the NXP LPC111x/LPC11Cxx/LPC11xxL/LPC11xxXL family of Cortex-M0 microcontrollers generated with the `svd2rust` tool.

## User Manual

The complete user manual for this family of microcontrollers may be found at [UM10398][1] and is a useful supplement to this crate.

## SVD Changes

The SVD file appears to have been generated from the user manual in an imperfect way and most of the enumerated values came out wrong e.g.:

```xml
<name>ENABLE_THE_THRE_INTE</name>
```

I've fixed up a significant number of these but only for the peripherals I have integrated into my projects; more fixes may be pushed over time. As far as I can tell the SVD file appears correct but as mentioned it is pretty gnarly to use in places. I would greatly appreciate pull requests with SVD fixes, ergonomic or otherwise, no matter how small.

## Contribute

Install the `svd2rust` and `form` tools and run the `generate.sh` script to generate the crate from the SVD file. A line is also prepended to the `lib.rs` to prevent Clippy from checking the crate because the generated code does not lint well. Do not make manual edits to the `src` folder or the `build.rs` and `device.x` files which are all auto-generated.

[1]: https://www.nxp.com/docs/en/user-guide/UM10398.pdf
