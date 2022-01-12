# Single Switch Guitar Pedal Firmware

This is a simple firmware intended for the ATtiny85 that operates any guitar pedal with a single footswitch.

Features:

- input debouncing
- press to toggle, hold for momentary operation

## Building

Like any Rust project, `cargo build` will compile the binaries.

Optionally, as a convenience, you can use `cargo run`, which will build and attempt to flash the board using `flash.sh`, which is specific to my toolchain and may need to be tweaked depending on your setup.

### Notes on compilation

While Rust has official support for the AVR architecture now, there's an [LLVM bug](https://reviews.llvm.org/D114611) that prevents it from properly compiling.
Unfortunately, this means we're stuck on a fairly old Rust nightly (I've been building with `1.51.0-nightly`).
See [this Github issue](https://github.com/rust-lang/rust/issues/82104) for more information.
