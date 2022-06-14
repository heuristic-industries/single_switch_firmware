# Single Switch Guitar Pedal Firmware

This is a simple firmware intended for the ATtiny85 that operates any guitar pedal with a single footswitch.

Features:

- input debouncing
- press to toggle, hold for momentary operation

## Building

Like any Rust project, `cargo build` will compile the binaries.

Optionally, as a convenience, you can use `cargo run`, which will build and attempt to flash the board using `flash.sh`, which is specific to my toolchain and may need to be tweaked depending on your setup.

### Notes on compilation

Due to an LLVM bug that broke the AVR target, this project requires any nightly after 2022-05-09 (when it [was fixed](https://github.com/rust-lang/rust/pull/96845)).
