# Waterman 🍉 
[![crates.io page](https://img.shields.io/crates/v/waterman.svg)](https://crates.io/crates/waterman)  

**The smoothest way to splash some Rust code into your AVR microcontrollers!** 🦀🔌  
Waterman is here to wrap around `avrdude` and make your Rust development for AVRs as easy as doing a cannonball into a pool on a hot day!  And if you’re used to the Arduino IDE, this tool's got you covered with that same easy access to your target's serial console. It's like diving head-first into coding bliss.

## Why Waterman?
Forget the command-line acrobatics. Waterman turns your `cargo run` into a one-stop shop: build, flash, run, repeat — like magic, but with less effort and more water! 💧

## Quick Fix for Serial Port Errors
Did Waterman give you an error about serial ports? No worries! If you see:
```
Error: no matching serial port found, use -P or set WATERMAN_PORT in your environment
```
Simply run `cargo run` with the environment variable set or adjust your `.cargo/config.toml` like so:
```toml
runner = "waterman {X} -cb {X} -P /dev/ttyUSB{X}" # Replace {X} with your actual values!
```
And you're good to dive back in!

## Installation 🛠️

Before you dive into the deep end, make sure you’ve got the right dependencies!

### Linux Pre-requisites:
- **Arch Linux:** `pacman -S systemd pkgconf`
- **Ubuntu/Debian:** `apt install libudev-dev pkg-config`
- **Fedora:** `dnf install systemd-devel pkgconf-pkg-config`

### Get Waterman Flowing 💧
To install Waterman from crates.io, just run:
```bash
cargo install waterman
```

Or, if you're a fan of Nix and Flakes, just add:
```nix
inputs.waterman.url = "github:cyberkutti-iedc/avr-hal?dir=waterman";
```
and use:
```nix
waterman.packages."${system}".default
```

## Integrating Waterman 🏗️

Time to set Waterman as your default "runner" for AVR projects! For example, if you’re working with an Arduino Uno, tweak your `.cargo/config.toml` (not `Cargo.toml`!) like this:
```toml
[target.'cfg(target_arch = "avr")']
runner = "waterman uno --open-console --baudrate 57600"
```
Now, every time you run `cargo run`, Waterman will handle building, flashing, and running your code with grace.

## Running in Style 💻🎉
Here’s what happens when you type `cargo run`:

```bash
cargo run --bin uno-i2cdetect
```

You'll see Waterman work its magic in the terminal like a synchronized swimmer:
```
Compiling arduino-uno-examples...
Finished dev [optimized + debuginfo] target(s) in 1.26s
Running waterman uno -cb 57600 avr-hal/target/avr-atmega328p/debug/uno-i2cdetect.elf
Board: Arduino Uno 🌊
Programming: avr-hal/target/avr-atmega328p/debug/uno-i2cdetect.elf 🧑‍💻 => /dev/ttyACM0

avrdude: Device signature = 0x1e950f
avrdude: Flashing with style... 💫
avrdude: Writing flash... Done in 0.27s!
avrdude: Verifying flash... Perfect match! 🧩

Programmed avr-hal/target/avr-atmega328p/debug/uno-i2cdetect.elf
Console: Now open at /dev/ttyACM0 (Baud: 57600) 📞

Running I2C detect tests...
```

## License 📜
Waterman is soaked in dual licenses:
- **Apache License, Version 2.0** ([LICENSE-APACHE](../LICENSE-APACHE) or [link](http://www.apache.org/licenses/LICENSE-2.0))
- **MIT license** ([LICENSE-MIT](../LICENSE-MIT) or [link](http://opensource.org/licenses/MIT))

Pick whichever you prefer, just like picking your favorite pool floatie! 🌞

---

Now, go and make a splash with Waterman! 🍉💧